#[macro_use]
extern crate serde;

use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = api.into_token_stream();
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

fn escape(s: &str) -> String {
    match s {
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
        _ => s.into()
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
        let declares = self.collect_declares();
        quote! {
            #[doc = #comment]
            #extends
            impl #name {
                #properties
                #methods
            }
            #declares
            #events
        }
    }

    fn name(&self) -> Ident {
        if self.name == "JSHandle" {
            return format_ident!("JsHandle");
        }
        format_ident!("{}", &self.name)
    }

    // fn extends(&self) -> Option<TokenStream> {
    //    self.extends.as_ref().map(|e| {
    //        let e = format_ident!("{}", e);
    //        quote! { :#e }
    //    })
    //}

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
        let labels = es.clone().map(|e| {
            let label = format_ident!("{}", e.body.name.to_camel());
            let comment = &e.body.comment;
            quote! {
                #[doc=#comment]
                #label
            }
        });
        let bodies = es.map(|e| {
            let label = format_ident!("{}", e.body.name.to_camel());
            let t = &e.body.r#type;
            let comment = &e.body.comment;
            quote! {
                #[doc=#comment]
                #label(#t)
            }
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

    fn collect_declares(&self) -> TokenStream {
        let mut res: TokenStream = quote! {};
        for member in &self.members {
            res.extend(member.r#type.declare(&member.name));
            for arg in member.args.iter().filter(|a| a.name != "options") {
                res.extend(arg.r#type.declare(&arg.name));
            }
        }
        res
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
            #[doc = #comment]
            fn #name(&self, #(#all),*) -> Result<#ty, #err> { todo!() }
        });
    }
}

impl Method<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", escape(&self.body.name.to_snake())) }
}

impl ToTokens for Property<'_, '_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name();
        let comment = &self.body.comment;
        let ty = &self.body.r#type;
        tokens.extend(quote! {
            #[doc = #comment]
            pub fn #name(&self) -> #ty {}
        });
    }
}

impl Property<'_, '_> {
    fn name(&self) -> Ident { format_ident!("{}", &self.body.name.to_snake()) }
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
                "Func" => todo!("{:?}", self),
                _ => unreachable!("{:?}", self)
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
                tokens.extend(quote! { Object });
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
                } else if n.starts_with('"') && n.ends_with('"') {
                    // TODO
                    format_ident!("{}", n[1..(n.len() - 1)])
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
    fn function(&self) -> TokenStream {
        let ret = self.return_type.as_ref().unwrap();
        quote! {
            impl Fn(NotImplementedYet) ->  #ret
        }
    }

    fn array(&self) -> TokenStream {
        let t = self.templates.iter().next().unwrap();
        quote! {
            Vec<#t>
        }
    }

    fn map(&self) -> TokenStream {
        let fst = self.templates.iter().next().unwrap();
        let snd = self.templates.iter().next().unwrap();
        quote! {
            Map<#fst, #snd>
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

    // TODO: recursive
    fn declare(&self, hint: &str) -> Option<TokenStream> {
        if !self.properties.is_empty() && self.name != "function" {
            let name = format_ident!("NotImplementedYet{}", hint);
            let required = self
                .properties
                .iter()
                .filter(|a| a.required)
                .map(|a| a.with_colon());
            let opts = self
                .properties
                .iter()
                .filter(|a| !a.required)
                .map(|a| a.with_colon_option());
            let all = required.chain(opts);
            let nested = self
                .properties
                .iter()
                .map(|a| a.r#type.declare(&a.name))
                .fold(quote! {}, |mut a, b| {
                    a.extend(b);
                    a
                });
            return Some(quote! {
                struct #name {
                    #(#all),*
                }
                #nested
            });
        } else if !self.union.is_empty() {
            let name = format_ident!("NotImplementedYet{}", hint);
            let not_null = self.union.iter().filter(|u| u.name != "null");
            if not_null.clone().count() <= 1 {
                return None;
            }
            let nested = not_null
                .clone()
                .map(|t| t.declare(""))
                .fold(quote! {}, |mut a, b| {
                    a.extend(b);
                    a
                });
            let xs = not_null.map(|t| {
                quote! { NotImplementedYet(#t) }
            });
            return Some(quote! {
                enum #name {
                    #(#xs),*
                }
                #nested
            });
        }
        None
    }
}

impl Arg {
    fn with_colon(&self) -> TokenStream {
        let name = self.name();
        let ty = &self.r#type;
        let comment = &self.comment;
        quote! {
            #[doc = #comment]
            #name: #ty
        }
    }

    fn with_colon_option(&self) -> TokenStream {
        let name = self.name();
        let ty = &self.r#type;
        let comment = &self.comment;
        quote! {
            #[doc = #comment]
            #name: Option<#ty>
        }
    }

    fn name(&self) -> Ident { format_ident!("{}", escape(&self.name.to_snake())) }
}

#[test]
fn case() {
    assert_eq!("handleSIGINT".to_snake(), "handle_s_i_g_i_n_t");
}
