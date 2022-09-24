use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use scripts::{
    api::{types::Model, *},
    utils
};
use std::collections::{HashMap, VecDeque};

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = to_tokens(&api);
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

fn to_tokens(api: &Api) -> TokenStream {
    let mut tokens = TokenStream::default();
    tokens.append_all(api.0.iter().map(body));
    tokens
}

fn body(x: &Interface) -> TokenStream {
    let name = format_ident!("{}", utils::loud_to_camel(&x.name));
    let mod_name = format_ident!("{}", utils::loud_to_camel(&x.name).to_snake());
    let extends = x.extends.as_deref().map(|e| {
        let e = format!("Extends {}", e);
        quote! { #[doc=#e] }
    });
    let (sub, methods, event) = types::collect_types(x);
    let sub = sub.into_iter().map(|m| format_ty(&*m));
    let mut overload_targets: HashMap<&str, Vec<&types::Method>> = methods
        .iter()
        .filter(|m| m.orig.overload_index > 0)
        .fold(HashMap::new(), |mut a, b| {
            a.entry(&*b.orig.alias)
                .and_modify(|xs| xs.push(b))
                .or_insert(vec![b]);
            a
        });
    let methods = methods
        .iter()
        .filter(|m| m.orig.overload_index == 0)
        .map(|m| {
            let overloads = overload_targets.remove(&*m.orig.alias);
            method_tokens(m, overloads)
        });
    let events = event.map(|event| event_tokens(event));
    quote! {
        mod #mod_name {
            #extends
            impl #name {
                #(#methods)*
            }
            #events
            #(#sub)*
        }
    }
}

fn format_ty(x: &types::Model) -> TokenStream {
    match x {
        Model::Struct {
            name,
            orig,
            fields,
            has_reference
        } => {
            let n = format_ident!("{}", name);
            let lifetime = has_reference.then(|| quote!(<'a>));
            let fields = fields.iter().map(|(k, v)| {
                let n = format_ident!("{}", k);
                let v = format_use_ty(v);
                quote! {
                    #n: #v
                }
            });
            quote! {
                struct #n #lifetime {
                    #(#fields),*
                }
            }
        }
        Model::Enum {
            name,
            orig,
            variants,
            has_reference
        } => {
            let n = format_ident!("{}", name);
            let lifetime = has_reference.then(|| quote!(<'a>));
            let variants = variants.iter().map(|variant| {
                let n = format_ident!("{}", variant.label);
                if let Some(x) = &variant.data {
                    let v = format_use_ty(x);
                    quote! {
                        #n (#v)
                    }
                } else {
                    quote! {
                        #n
                    }
                }
            });
            quote! {
                enum #n #lifetime {
                    #(#variants),*
                }
            }
        }
        _ => {
            quote! {}
        }
    }
}

fn format_use_ty(x: &types::Model) -> TokenStream {
    let reference = x.has_reference();
    let lifetime = reference.then(|| quote!(<'a>));
    match x {
        Model::Struct { name, .. } => {
            let n = format_ident!("{}", name);
            quote! {#n #lifetime}
        }
        Model::Enum { name, .. } => {
            let n = format_ident!("{}", name);
            quote! {#n #lifetime}
        }
        Model::Option(y) => {
            let y = format_use_ty(y);
            quote!(Option<#y>)
        }
        Model::Vec(y) => {
            let y = format_use_ty(y);
            quote!(Vec<#y>)
        }
        Model::Map(y, z) => {
            let y = format_use_ty(y);
            let z = format_use_ty(z);
            quote!(HashMap<#y, #z>)
        }
        Model::Known { name, .. } => match name.as_ref() {
            "binary" if reference => quote!(&'a [u8]),
            "binary" => quote!(Vec<u8>),
            "json" if reference => quote!(&'a str),
            "json" => quote!(String),
            "string" if reference => quote!(&'a str),
            "string" => quote!(String),
            "number" => quote!(serde_json::Number),
            "float" => quote!(f64),
            "boolean" => quote!(bool),
            "void" => quote!(()),
            _ => {
                let n = format_ident!("{}", name);
                assert!(!reference);
                quote!(#n)
            }
        }
    }
}

fn event_tokens(event: types::Event) -> TokenStream {
    let types::Event { orig, model, which } = event;
    let model = format_ty(&*model);
    let which = format_ty(&*which);
    quote! {
        #model
        #which
    }
}

fn method_tokens(method: &types::Method, overloads: Option<Vec<&types::Method>>) -> TokenStream {
    let types::Method {
        orig:
            Member {
                kind: _,
                name,
                alias,
                experimental,
                since: _,
                overload_index: _,
                required,
                is_async,
                args: member_args,
                ty: member_ty,
                deprecated,
                spec // TODO
            },
        args,
        builder,
        ty
    } = method;
    let is_builder = types::needs_builder(&method.orig);
    assert!(name == alias || name.starts_with(alias), "{}", name);
    let rety = format_use_ty(builder.as_deref().unwrap_or(&*ty));
    let arg_fields = args.iter().map(|(name, model)| {
        let name = format_ident!("{}", name);
        let ty = format_use_ty(model);
        quote! {
            #name: #ty
        }
    });
    let fn_name = if is_builder {
        format_ident!(
            "{}_builder",
            utils::loud_to_camel(&name.replace("#", "")).to_snake()
        )
    } else {
        format_ident!("{}", utils::loud_to_snake(&name.replace("#", "")))
    };
    let mark_async = (!is_builder && *is_async)
        .then(|| quote!(async))
        .unwrap_or_default();
    let doc_unnecessary = (!required)
        .then(|| quote!(#[doc="unnecessary"]))
        .unwrap_or_default();
    let doc_experimental = experimental
        .then(|| quote!(#[doc="experimental"]))
        .unwrap_or_default();
    let mark_deprecated = deprecated
        .then(|| quote!(#[deprecated]))
        .unwrap_or_default();
    quote! {
        #doc_unnecessary
        #doc_experimental
        #mark_deprecated
        #mark_async fn #fn_name() -> #rety {
            todo!()
        }
    }
}
