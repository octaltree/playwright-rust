#[macro_use]
extern crate serde;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde_json::Value;

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = api.into_token_stream();
    println!("{}", t);
}

fn escape(s: String) -> String {
    match s.as_str() {
        // keywords https://doc.rust-lang.org/book/appendix-01-keywords.html
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else"
        | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "union" | "unsafe"
        | "use" | "where" | "while" => {
            format!("r#{}", s)
        }
        // reserved
        "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "try"
        | "typeof" | "unsized" | "virtual" | "yield" => {
            format!("r#{}", s)
        }
        _ => s
    }
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
    r#type: Type,
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

    fn name(&self) -> Ident {
        if self.name == "JSHandle" {
            return format_ident!("JsHandle");
        }
        format_ident!("{}", &self.name)
    }

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
        let ms = self
            .members
            .iter()
            .filter(|m| m.kind == Kind::Method)
            .map(|m| Method {
                name: &self.name,
                body: m
            });
        quote! {
            #(#ms)*
        }
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name();
        let comment = &self.body.comment;
        let ty = &self.body.r#type;
        let err = if self.body.is_async {
            quote! {Arc<Error>}
        } else {
            quote! {Error}
        };
        tokens.extend(quote! {
            //TODO:#[doc = #comment]
            pub fn #name() -> Result<#ty, #err> {}
        });
    }
}

impl Method<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", escape(self.body.name.to_case(Case::Snake))) }
}

impl ToTokens for Property<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name();
        let comment = &self.body.comment;
        let ty = &self.body.r#type;
        tokens.extend(quote! {
            //TODO:#[doc = #comment]
            pub fn #name() -> #ty {}
        });
    }
}

impl Property<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", &self.body.name.to_case(Case::Snake)) }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.properties.is_null() {
            tokens.extend(quote! {
                NotImplementedYet
            });
            return;
        }
        if !self.return_type.is_null() {
            tokens.extend(quote! {
                NotImplementedYet
            });
            return;
        }
        if !self.templates.is_null() {
            tokens.extend(quote! {
                NotImplementedYet
            });
            return;
        }
        if !self.union.is_null() {
            tokens.extend(quote! {
                NotImplementedYet
            });
            return;
        }
        match self.name.as_str() {
            "" => {
                unreachable!()
            }
            "Object" => {
                self.object(tokens);
                return;
            }
            "void" => {
                tokens.extend(quote! { () });
                return;
            }
            "string" => {
                tokens.extend(quote! { String });
                return;
            }
            "boolean" => {
                tokens.extend(quote! { bool });
                return;
            }
            "JSHandle" => {
                tokens.extend(quote! { JsHandle });
                return;
            }
            "int" => {
                tokens.extend(quote! { i64 });
                return;
            }
            // any
            // Any
            n => {
                let name = if n == "System.Net.HttpStatusCode" {
                    format_ident!("i32")
                } else {
                    format_ident!("{}", n)
                };
                tokens.extend(quote! {
                    #name
                });
                return;
            }
        }
    }
}

impl Type {
    fn object(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            Object
        });
    }
}
