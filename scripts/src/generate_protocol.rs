use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use scripts::{protocol::*, utils};
use std::collections::HashMap;

fn main() {
    let protocol: Protocol = serde_yaml::from_reader(std::io::stdin()).unwrap();
    let t = to_tokens(&protocol);
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

fn to_tokens(protocol: &Protocol) -> TokenStream {
    let mut xs = protocol.0.iter().collect::<Vec<_>>();
    xs.sort_by_key(|(_, n)| match n {
        Node::Interface(_) => 1,
        Node::Object(_) => 2,
        // partial type of object
        Node::Mixin(_) => 3,
        // sum type is compound but literals is primitive
        Node::Enum(_) => 4
    });
    let mut tokens = TokenStream::default();
    tokens.append_all(xs.into_iter().map(|(name, node)| node_tokens(name, node)));
    tokens
}

fn node_tokens(name: &str, node: &Node) -> TokenStream {
    match node {
        Node::Enum(x) => enum_tokens(name, x, true),
        Node::Object(x) | Node::Mixin(x) => object_tokens(name, x),
        Node::Interface(x) => interface_tokens(name, x)
    }
}

fn enum_tokens(name: &str, x: &Enum, camel: bool) -> TokenStream {
    let variants = x
        .literals
        .iter()
        .map(|s| {
            let (variant, is_normalized) = {
                let u = s.replace("-", "_");
                let snake = if u.starts_with("_") {
                    format!("neg{}", u)
                } else {
                    u
                };
                let raw = if camel { snake.to_camel() } else { snake };
                (format_ident!("{}", raw), s != &raw)
            };
            let orig = is_normalized
                .then(|| quote!(#[rename=#s]))
                .unwrap_or_default();
            quote! {
                #orig
                #variant
            }
        })
        .collect::<Vec<_>>();
    let name = format_ident!("{}", name);
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn object_tokens(name: &str, x: &Object) -> TokenStream {
    let nodes = collect_unnamed_by_properties(vec![], &x.properties);
    let mut once = nodes
        .iter()
        .map(|(name, node)| node_tokens(name, node))
        .peekable();
    let struct_name = format_ident!("{}", name);
    let sub = if once.peek().is_some() {
        let mod_name = format_ident!("{}", name.to_snake());
        quote! {
            pub mod #mod_name {
                #(#once)*
            }
        }
    } else {
        Default::default()
    };
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #struct_name {
        }

        #sub
    }
}

fn interface_tokens(name: &str, x: &Interface) -> TokenStream {
    let Interface {
        commands,
        events,
        extends,
        initializer
    } = x;
    let mod_name = format_ident!("{}", utils::fix_loud_camel(name).to_snake());
    // FIXME: duplicated
    let initializer_tokens = initializer
        .clone()
        .map(|properties| object_tokens("initializer", &Object { properties }))
        .unwrap_or_default();
    quote! {
        pub mod #mod_name {
            #initializer_tokens
        }
    }
}
