#[macro_use]
extern crate serde;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde_json::Value;

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let mut all_types = Vec::new();
    for i in api.0 {
        for m in i.members {
            all_types.push(m.r#type.clone());
            for a in m.args {
                all_types.push(a.r#type.clone());
            }
        }
    }
    println!("{}", serde_json::to_string(&all_types).unwrap());
    // let t = api.into_token_stream();
    // println!("{}", t);
}

#[derive(Debug, Deserialize)]
struct Api(Vec<Interface>);

#[derive(Debug, Deserialize)]
struct Interface {
    name: String,
    // langs
    // spec
    #[serde(default)]
    comment: String,
    members: Vec<Member>,
    extends: Option<String>
}

#[derive(Debug, Deserialize)]
struct Member {
    kind: Kind,
    name: String,
    // langs
    // alias
    r#type: Value,
    // spec
    #[serde(default)]
    comment: String,
    required: bool,
    deprecated: bool,
    #[serde(rename = "async")]
    is_async: bool,
    args: Vec<Arg>
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
enum Kind {
    Event,
    Method,
    Property
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Type {
    // 30 ('event', dict_keys(['name', 'expression']))
    //  2 ('event', dict_keys(['name', 'properties', 'expression']))
    //  4 ('event', dict_keys(['name']))
    // 151 ('method', dict_keys(['name', 'expression']))
    //  3 ('method', dict_keys(['name', 'properties', 'expression']))
    // 23 ('method', dict_keys(['name', 'templates', 'expression']))
    // 44 ('method', dict_keys(['name', 'union', 'expression']))
    // 132 ('method', dict_keys(['name']))
    // 10 ('property', dict_keys(['name', 'expression']))
    //  1 ('property', dict_keys(['name', 'properties', 'expression']))
    //  1 ('property', dict_keys(['name', 'union', 'expression']))

    //  9 dict_keys(['name', 'args', 'returnType', 'expression'])
    // 220 dict_keys(['name', 'expression'])
    // 10 dict_keys(['name', 'properties', 'expression'])
    // 150 dict_keys(['name', 'properties'])
    //  9 dict_keys(['name', 'templates', 'expression'])
    // 54 dict_keys(['name', 'union', 'expression'])
    name: String,
    #[serde(default)]
    return_type: serde_json::Value,
    #[serde(default)]
    expression: Option<String>,
    #[serde(default)]
    properties: serde_json::Value,
    #[serde(default)]
    templates: serde_json::Value,
    #[serde(default)]
    union: serde_json::Value
}

#[derive(Debug, Deserialize)]
struct Arg {
    name: String,
    kind: ArgKind,
    // langs
    // alias
    r#type: Value,
    // spec
    comment: String,
    required: bool,
    deprecated: bool,
    #[serde(rename = "async")]
    is_async: bool
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
enum ArgKind {
    Property
}

impl ToTokens for Api {
    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.append_all(&self.0); }
}
// TODO:EventEmitter, Timeout

impl ToTokens for Interface {
    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.body()); }
}

impl Interface {
    fn body(&self) -> TokenStream {
        let name = self.name();
        let extends = self.extends();
        let comment = &self.comment;
        let methods = self.methods();
        let properties = self.properties();
        let events = self.events();
        quote! {
            //TODO:#[doc = #comment]
            impl #name {
                #properties
                #methods
            }
            #events
        }
    }

    fn name(&self) -> Ident { format_ident!("{}", &self.name) }

    fn extends(&self) -> Option<TokenStream> {
        self.extends.as_ref().map(|e| {
            let e = format_ident!("{}", e);
            quote! { :#e }
        })
    }

    fn properties(&self) -> TokenStream {
        let ps = self
            .members
            .iter()
            .filter(|m| m.kind == Kind::Property)
            .map(|m| Property {
                name: &self.name,
                body: m
            });
        quote! {
            #(#ps)*
        }
    }

    fn methods(&self) -> TokenStream {
        quote! {}
    }

    fn events(&self) -> TokenStream {
        quote! {}
    }
}

struct Event<'a, 'b> {
    name: &'a str,
    body: &'b Member
}
struct Method<'a, 'b> {
    name: &'a str,
    body: &'b Member
}
struct Property<'a, 'b> {
    name: &'a str,
    body: &'b Member
}

impl ToTokens for Event<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream) { todo!() }
}

impl ToTokens for Method<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream) { todo!() }
}

impl ToTokens for Property<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name();
        let comment = &self.body.comment;
        // let ty = &self.body.r#type;
        tokens.extend(quote! {
            //TODO:#[doc = #comment]
            pub fn #name() {}
        });
    }
}

impl Property<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", &self.body.name.to_case(Case::Snake)) }
}

// impl ToTokens for Type {
//    fn to_tokens(&self, tokens: &mut TokenStream) {
//        match self.name.as_str() {
//            "" => {
//                // unimplemented!()
//                tokens.extend(quote! {
//                    ()
//                });
//                return;
//            }
//            "Object" => {
//                tokens.extend(quote! {
//                    Value
//                });
//                return;
//            }
//            n => {
//                let name = format_ident!("{}", n);
//                tokens.extend(quote! {
//                    #name
//                });
//                return;
//            }
//        }
//    }
//}
