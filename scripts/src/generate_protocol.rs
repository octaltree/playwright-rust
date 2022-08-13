use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use scripts::{protocol::*, utils};

fn main() {
    let protocol: Protocol = serde_yaml::from_reader(std::io::stdin()).unwrap();
    let t = to_tokens(&protocol);
    let g = quote! {
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(transparent)]
        pub struct Guid(String);
        pub type Channel = Guid;
    };
    println!("{}\n{}\n// vim: foldnestmax=0 ft=rust", g, t);
}

fn to_tokens(protocol: &Protocol) -> TokenStream {
    let mut xs = protocol.0.iter().collect::<Vec<_>>();
    xs.sort_by_key(|&(name, n)| match n {
        Node::Interface(_) => (1, name),
        Node::Object(_) => (2, name),
        // partial type of object
        Node::Mixin(_) => (3, name),
        // sum type is compound but literals is primitive
        Node::Enum(_) => (4, name)
    });
    let mut tokens = TokenStream::default();
    tokens.append_all(xs.into_iter().map(|(name, node)| node_tokens(name, node)));
    tokens
}

fn node_tokens(name: &str, node: &Node) -> TokenStream {
    match node {
        Node::Enum(x) => enum_tokens(name, x),
        Node::Object(x) | Node::Mixin(x) => object_tokens(name, x),
        Node::Interface(x) => interface_tokens(name, x)
    }
}

fn enum_tokens(name: &str, x: &Enum) -> TokenStream {
    let variants = x
        .literals
        .iter()
        .map(|s| {
            let (variant, is_normalized) = {
                let raw = utils::kebab_to_camel(s);
                (format_ident!("{}", raw), s != &raw)
            };
            let orig = is_normalized
                .then(|| quote!(#[serde(rename=#s)]))
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

/// for rooted named object
fn object_tokens(name: &str, x: &Object) -> TokenStream { child_object_tokens(name, &x.properties) }

/// for inner unnamed object
/// recursion of declare_object
fn child_object_tokens(name: &str, x: &Properties) -> TokenStream {
    let declare = declare_object(name, x);
    let x = {
        let mut x = x.iter().collect::<Vec<_>>();
        x.sort_by_cached_key(|&(name, _)| name);
        x
    };
    let sub = x
        .iter()
        .map(|&(field_name, ty)| declare_ty(name, field_name, ty));
    quote! {
        #declare
        #(#sub)*
    }
}

fn declare_object(name: &str, x: &Properties) -> TokenStream {
    let x = {
        let mut x = x.iter().collect::<Vec<_>>();
        x.sort_by_cached_key(|&(name, _)| name);
        x
    };
    let struct_name = format_ident!("{}", utils::loud_to_camel(name));
    let fields = x.iter().map(|(field_name, ty)| {
        let label = format_ident!(
            "{}",
            utils::loud_to_snake(&field_name.replace("$mixin", "mixin"))
        );
        let flatten = field_name
            .contains("$mixin")
            .then(|| quote!(#[serde(flatten)]))
            .unwrap_or_default();
        let use_ty = use_ty(name, field_name, ty);
        quote! {
            #flatten
            #[serde(rename = #field_name)]
            #label: #use_ty
        }
    });
    quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct #struct_name {
            #(#fields),*
        }
    }
}

fn use_ty(scope: &str, name: &str, ty: &Type) -> TokenStream {
    match ty {
        Type::Name(s) => {
            let opt = s.ends_with("?");
            let s = s.replace("?", "");
            let label = match &*s {
                "binary" => quote!(Vec<u8>),
                "number" => quote!(serde_json::Number),
                "json" => quote!(String),
                "string" => quote!(String),
                "boolean" => quote!(bool),
                x => {
                    let ident = format_ident!("{}", utils::loud_to_camel(x));
                    quote!(crate::protocol::generated::#ident)
                }
            };
            if opt {
                quote!(Option<#label>)
            } else {
                quote!(#label)
            }
        }
        Type::Items { r#type, item_type } => {
            let l = use_ty(scope, name, item_type);
            if r#type.ends_with("?") {
                quote!(Option<Vec<#l>>)
            } else {
                quote!(Vec<#l>)
            }
        }
        Type::Literals { r#type, .. } | Type::Properties { r#type, .. } => {
            let label = format_ident!(
                "{}{}",
                utils::loud_to_camel(scope),
                utils::lower_loud_to_camel(name)
            );
            if scope == "AndroidSelector"
                && (name == "hasChild" || name == "hasDescendant")
                && r#type.ends_with("?")
            {
                quote! {
                    Option<Box<#label>>
                }
            } else if r#type.ends_with("?") {
                quote!(Option<#label>)
            } else {
                quote!(#label)
            }
        }
    }
}

fn declare_ty(scope: &str, name: &str, ty: &Type) -> TokenStream {
    let label = format!(
        "{}{}",
        utils::loud_to_camel(scope),
        utils::lower_loud_to_camel(name)
    );
    match ty {
        Type::Name(_) => quote! {},
        Type::Items { r#type, item_type } => {
            assert!(r#type == "array" || r#type == "array?", "{}", &r#type);
            declare_ty(scope, name, item_type)
        }
        Type::Literals { r#type, literals } => {
            assert!(r#type == "enum" || r#type == "enum?", "{}", &r#type);
            enum_tokens(
                &label,
                &Enum {
                    literals: literals.clone()
                }
            )
        }
        Type::Properties { r#type, properties } => {
            assert!(r#type == "object" || r#type == "object?", "{}", &r#type);
            child_object_tokens(&label, properties)
        }
    }
}

fn interface_tokens(name: &str, x: &Interface) -> TokenStream {
    let Interface {
        commands,
        events,
        extends,
        initializer
    } = x;
    let mod_name = format_ident!("{}", utils::loud_to_camel(name).to_snake());
    let initializer_tokens = initializer
        .as_ref()
        .map(|properties| child_object_tokens("Initializer", &properties))
        .unwrap_or_default();
    let commands_tokens = commands_tokens(commands);
    let events_tokens = events_tokens(events);
    let doc_extends = extends
        .as_deref()
        .filter(|&s| s != "EventTarget")
        .map(|s| {
            let e = format!("Extends {}", s);
            quote! { #[doc=#e] }
        })
        .unwrap_or_default();
    let struct_name = format_ident!("{}", utils::loud_to_camel(name));
    quote! {
        #doc_extends
        pub mod #mod_name {
            #initializer_tokens

            #events_tokens
            #commands_tokens
        }

        pub type #struct_name = Guid;
    }
}

fn commands_tokens(commands: &Option<Commands>) -> TokenStream {
    let commands = match commands {
        Some(x) => {
            let mut tmp = x.iter().collect::<Vec<_>>();
            tmp.sort_by_cached_key(|&(name, _)| name);
            tmp
        }
        None => return Default::default()
    };
    let declares = commands.iter().map(|(name, c)| {
        let camel = utils::lower_loud_to_camel(name);
        let rety = format_ident!("{camel}");
        let args = format_ident!("{camel}Args");
        let c = match c.as_ref() {
            None => {
                return quote! {
                    pub type #rety = ();
                    pub type #args = ();
                }
            }
            Some(c) => c
        };
        let declare_rety = if let Some(returns) = &c.returns {
            child_object_tokens(&camel, returns)
        } else {
            quote! {
                pub type #rety = ();
            }
        };
        let declare_args = if let Some(parameters) = &c.parameters {
            child_object_tokens(&format!("{camel}Args"), parameters)
        } else {
            quote! {
                pub type #args = ();
            }
        };
        quote! {
            #declare_rety
            #declare_args
        }
    });
    quote! {
        pub mod commands {
            #(#declares)*
        }
    }
}

fn events_tokens(events: &Option<Events>) -> TokenStream {
    let events = match events {
        Some(x) => {
            let mut tmp = x.iter().collect::<Vec<_>>();
            tmp.sort_by_cached_key(|&(name, _)| name);
            tmp
        }
        None => return quote! {}
    };
    let variants = events.iter().map(|(name, e)| {
        let camel = utils::lower_loud_to_camel(name);
        let label = format_ident!("{}", &camel);
        if let Some(_) = e {
            quote! {
                #[serde(rename = #name)]
                #label(#label)
            }
        } else {
            quote! {
                #[serde(rename = #name)]
                #label
            }
        }
    });
    let sub = events.iter().filter_map(|(name, e)| -> Option<_> {
        let e = e.as_ref()?;
        let camel = utils::lower_loud_to_camel(name);
        let declare = child_object_tokens(&camel, &e.parameters);
        Some(quote! {
            #declare
        })
    });
    quote! {
        pub mod events {
            #[derive(Debug, Deserialize, Serialize)]
            pub enum Events {
                #(#variants),*
            }
            #(#sub)*
        }
    }
}
