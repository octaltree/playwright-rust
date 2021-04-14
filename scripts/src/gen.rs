#[macro_use]
extern crate serde;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde_json::Value;

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = api.into_token_stream();
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
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

    //  9 dict_keys(['name', 'args', 'returnType', 'expression'])
    // 220 dict_keys(['name', 'expression'])
    // 10 dict_keys(['name', 'properties', 'expression'])
    // 150 dict_keys(['name', 'properties'])
    //  9 dict_keys(['name', 'templates', 'expression'])
    // 54 dict_keys(['name', 'union', 'expression'])
    name: String,
    #[serde(default)]
    return_type: Option<Box<Type>>,
    #[serde(default)]
    expression: Option<String>,
    #[serde(default)]
    properties: Vec<Arg>,
    #[serde(default)]
    templates: Vec<Type>,
    #[serde(default)]
    union: Vec<Type>
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

impl ToTokens for Interface {
    fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.body()); }
}

impl Interface {
    fn body(&self) -> TokenStream {
        let name = self.name();
        let extends = self.extends.as_deref().map(|e| {
            let e = format!("Extends {}", e);
            quote! { #[doc=#e] }
        });
        let comment = &self.comment;
        let methods = self.methods();
        let properties = self.properties();
        let events = self.events();
        quote! {
            //TODO:#[doc = #comment]
            #extends
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
        let es = self
            .members
            .iter()
            .filter(|x| x.kind == Kind::Event)
            .map(|x| Event {
                name: &self.name,
                body: x
            });
        if es.clone().next().is_none() {
            return quote! {};
        }
        let labels = es
            .clone()
            .map(|e| format_ident!("{}", e.body.name.to_case(Case::UpperCamel)));
        let bodies = es.map(|e| {
            let label = format_ident!("{}", e.body.name.to_case(Case::UpperCamel));
            let t = &e.body.r#type;
            quote! { #label(#t) }
        });
        let et = format_ident!("{}EventType", self.name);
        let e = format_ident!("{}Event", self.name);
        quote! {
            enum #et {
                #(#labels),*
            }
            enum #e {
                #(#bodies),*
            }
        }
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
        let required = self
            .body
            .args
            .iter()
            .filter(|a| a.required)
            .map(|a| a.with_colon());
        let opts = self
            .body
            .args
            .iter()
            .filter(|a| !a.required && a.name != "options")
            .map(|a| a.with_colon_option());
        let options = self
            .body
            .args
            .iter()
            .filter(|a| !a.required && a.name == "options")
            .map(|a| {
                let xs = a.r#type.properties.iter().map(|a| a.with_colon_option());
                quote! { #[doc = "options"] #(#xs),* }
            });
        let all = required.chain(opts).chain(options);
        tokens.extend(quote! {
            //TODO:#[doc = #comment]
            pub fn #name(&self, #(#all),*) -> Result<#ty, #err> { todo!() }
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
            pub fn #name(&self) -> #ty {}
        });
    }
}

impl Property<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", &self.body.name.to_case(Case::Snake)) }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.return_type.is_some() {
            tokens.extend(self.function());
            return;
        }
        if !self.templates.is_empty() {
            tokens.extend(match self.name.as_str() {
                "Array" => self.array(),
                "Object" | "Map" => self.map(),
                _ => unreachable!()
            });
            return;
        }
        if !self.properties.is_empty() {
            if self.name.is_empty() || self.name == "Object" {
                tokens.extend(quote! { NotImplementedYet });
            } else {
                unreachable!();
            }
            return;
        }
        if !self.union.is_empty() {
            let optional = self.union.iter().find(|u| u.name == "null");
            if self.name.is_empty() {
                let t = self.r#enum();
                tokens.extend(optional.map(|_| quote! { Option<#t> }).unwrap_or(t));
            } else {
                let name = format_ident!("{}", self.name);
                let name = quote! { #name };
                tokens.extend(optional.map(|_| quote! { Option<#name> }).unwrap_or(name));
            }
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
            "float" => {
                tokens.extend(quote! { f64 });
                return;
            }
            // any Any Serializable path
            n => {
                let name = if n == "System.Net.HttpStatusCode" {
                    format_ident!("u16")
                } else if n == r#""gone""# {
                    format_ident!("Gone")
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

    fn function(&self) -> TokenStream {
        quote! {
            NotImplementedYet
        }
    }

    fn array(&self) -> TokenStream {
        quote! {
            NotImplementedYet
        }
    }

    fn map(&self) -> TokenStream {
        quote! {
            NotImplementedYet
        }
    }

    fn r#enum(&self) -> TokenStream {
        let mut entries = self.union.iter().filter(|u| u.name != "null");
        let num = entries.clone().count();
        match num {
            0 => unreachable!(),
            1 => {
                let first = entries.next().unwrap();
                quote! { #first }
            }
            _ => {
                quote! {
                    NotImplementedYet
                }
            }
        }
    }
}

impl Arg {
    fn with_colon(&self) -> TokenStream {
        let name = self.name();
        let ty = &self.r#type;
        quote! {
            #name: #ty
        }
    }

    fn with_colon_option(&self) -> TokenStream {
        let name = self.name();
        let ty = &self.r#type;
        quote! {
            #name: Option<#ty>
        }
    }

    fn name(&self) -> Ident { format_ident!("{}", escape(self.name.to_case(Case::Snake))) }
}
