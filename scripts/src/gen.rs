#[macro_use]
extern crate serde;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use std::fmt;

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = api.into_token_stream();
    println!("{}", t);
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
    r#type: Type,
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

#[derive(Debug, Deserialize)]
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
    name: String,
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
    // 150 dict_keys(['kind', 'langs', 'name', 'type', 'required', 'comment', 'deprecated', 'async', 'alias'])
    // 302 dict_keys(['kind', 'langs', 'name', 'type', 'spec', 'required', 'comment', 'deprecated', 'async', 'alias'])
    name: String,
    kind: ArgKind,
    // langs
    // alias
    r#type: ArgType,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArgType {
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

impl ToTokens for Api {
    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.append_all(&self.0); }
}

impl ToTokens for Interface {
    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(quote::quote! {}); }
}
