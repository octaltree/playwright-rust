use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};
use scripts::{api::*, utils};

fn main() {
    let api: Api = serde_json::from_reader(std::io::stdin()).unwrap();
    let t = to_tokens(&api);
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

fn to_tokens(api: &Api) -> TokenStream {
    let mut tokens = TokenStream::default();
    tokens.append_all(api.0.iter().map(body));
    tokens.extend(collect_types(&api.0));
    tokens
}

fn body(x: &Interface) -> TokenStream {
    let name = format_ident!("{}", utils::fix_loud_camel(&x.name));
    let mod_name = format_ident!("{}", utils::fix_loud_camel(&x.name).to_snake());
    let extends = x.extends.as_deref().map(|e| {
        let e = format!("Extends {}", e);
        quote! { #[doc=#e] }
    });
    // let properties = self.properties();
    quote! {
        mod #mod_name {
            #extends
            impl #name {
            }
        }
    }
}

fn collect_types(xs: &[Interface]) -> TokenStream {
    quote! {}
}

// impl Interface {
//     fn body(&self) -> TokenStream {
//        let name = self.name();
//        let extends = self.extends.as_deref().map(|e| {
//            let e = format!("Extends {}", e);
//            quote! { #[doc=#e] }
//        });
//        let comment = &self.comment;
//        let methods = self.methods();
//        let properties = self.properties();
//        let events = self.events();
//        let declares = self.collect_declares();
//        quote! {
//            #[doc = #comment]
//            #extends
//            impl #name {
//                #properties
//                #methods
//            }
//            #declares
//            #events
//        }
//    }

//     fn name(&self) -> Ident { format_ident!("{}", fix_loud_camel(&self.name)) }

//     fn extends(&self) -> Option<TokenStream> {
//        self.extends.as_ref().map(|e| {
//            let e = format_ident!("{}", e);
//            quote! { :#e }
//        })
//     }

//     fn properties(&self) -> TokenStream {
//        let ps = self
//            .members
//            .iter()
//            .filter(|m| m.kind == Kind::Property)
//            .map(|m| Property {
//                name: &self.name,
//                body: m
//            });
//        quote! {
//            #(#ps)*
//        }
//    }

//     fn methods(&self) -> TokenStream {
//        let ms = self
//            .members
//            .iter()
//            .filter(|m| m.kind == Kind::Method)
//            .map(|m| Method {
//                name: &self.name,
//                body: m
//            });
//        quote! {
//            #(#ms)*
//        }
//    }

//     fn events(&self) -> TokenStream {
//        let es = self
//            .members
//            .iter()
//            .filter(|x| x.kind == Kind::Event)
//            .map(|x| Event {
//                name: &self.name,
//                body: x
//            });
//        if es.clone().next().is_none() {
//            return quote! {};
//        }
//        let labels = es.clone().map(|e| {
//            let label = format_ident!("{}", e.body.name.to_camel());
//            let comment = &e.body.comment;
//            quote! {
//                #[doc=#comment]
//                #label
//            }
//        });
//        let bodies = es.map(|e| {
//            let label = format_ident!("{}", e.body.name.to_camel());
//            let t = &e.body.ty;
//            let comment = &e.body.comment;
//            quote! {
//                #[doc=#comment]
//                #label(#t)
//            }
//        });
//        let et = format_ident!("{}EventType", self.name);
//        let e = format_ident!("{}Event", self.name);
//        quote! {
//            enum #et {
//                #(#labels),*
//            }
//            enum #e {
//                #(#bodies),*
//            }
//        }
//    }

//     fn collect_declares(&self) -> TokenStream {
//        let mut res: TokenStream = quote! {};
//        for member in &self.members {
//            res.extend(member.ty.declare(&member.name));
//            for arg in member.args.iter().filter(|a| a.name != "options") {
//                res.extend(arg.ty.declare(&arg.name));
//            }
//        }
//        res
//    }
//}

// struct Event<'a, 'b> {
//    name: &'a str,
//    body: &'b Member
//}
// struct Method<'a, 'b> {
//    name: &'a str,
//    body: &'b Member
//}
// struct Property<'a, 'b> {
//    name: &'a str,
//    body: &'b Member
//}

// impl ToTokens for Method<'_, '_> {
//    fn to_tokens(&self, tokens: &mut TokenStream) {
//        let name = self.name();
//        let comment = &self.body.comment;
//        let ty = &self.body.ty;
//        let err = if self.body.is_async {
//            quote! {Arc<Error>}
//        } else {
//            quote! {Error}
//        };
//        let required = self
//            .body
//            .args
//            .iter()
//            .filter(|a| a.required)
//            .map(|a| a.with_colon());
//        let opts = self
//            .body
//            .args
//            .iter()
//            .filter(|a| !a.required && a.name != "options")
//            .map(|a| a.with_colon_option());
//        let options = self
//            .body
//            .args
//            .iter()
//            .filter(|a| !a.required && a.name == "options")
//            .map(|a| {
//                let xs = a.ty.properties.iter().map(|a| a.with_colon_option());
//                quote! { #[doc = "options"] #(#xs),* }
//            });
//        let all = required.chain(opts).chain(options);
//        tokens.extend(quote! {
//            #[doc = #comment]
//            fn #name(&self, #(#all),*) -> Result<#ty, #err> { todo!() }
//        });
//    }
//}

// impl Method<'_, '_> {
//    fn name(&self) -> Ident { format_ident!("{}", escape(&self.body.name.to_snake())) }
//}

// impl ToTokens for Property<'_, '_> {
//    fn to_tokens(&self, tokens: &mut TokenStream) {
//        let name = ident_snake(&self.body.name);
//        let comment = &self.body.comment;
//        let ty = &self.body.ty;
//        tokens.extend(quote! {
//            #[doc = #comment]
//            pub fn #name(&self) {}
//        });
//    }
//}

// fn ident_snake(name: &str) -> Ident { format_ident!("{}", escape(&name.to_snake())) }

// impl ToTokens for Type {
//    fn to_tokens(&self, tokens: &mut TokenStream) {
//        if !self.templates.is_empty() {
//            tokens.extend(match self.name.as_str() {
//                "Array" => self.array(),
//                "Object" | "Map" => self.map(),
//                "Func" => todo!("{:?}", self),
//                _ => unreachable!("{:?}", self)
//            });
//            return;
//        }
//        if !self.properties.is_empty() {
//            if self.name.is_empty() || self.name == "Object" {
//                tokens.extend(quote! { NotImplementedYet });
//            } else {
//                unreachable!();
//            }
//            return;
//        }
//        if !self.union.is_empty() {
//            let optional = self.union.iter().find(|u| u.name == "null");
//            if self.name.is_empty() {
//                let t = self.r#enum();
//                tokens.extend(optional.map(|_| quote! { Option<#t> }).unwrap_or(t));
//            } else {
//                let name = format_ident!("{}", self.name);
//                let name = quote! { #name };
//                tokens.extend(optional.map(|_| quote! { Option<#name> }).unwrap_or(name));
//            }
//            return;
//        }
//        match self.name.as_str() {
//            "" => {
//                unreachable!()
//            }
//            "Object" => {
//                tokens.extend(quote! { Object });
//                return;
//            }
//            "void" => {
//                tokens.extend(quote! { () });
//                return;
//            }
//            "string" => {
//                tokens.extend(quote! { String });
//                return;
//            }
//            "boolean" => {
//                tokens.extend(quote! { bool });
//                return;
//            }
//            "JSHandle" => {
//                tokens.extend(quote! { JsHandle });
//                return;
//            }
//            "int" => {
//                tokens.extend(quote! { i64 });
//                return;
//            }
//            "float" => {
//                tokens.extend(quote! { f64 });
//                return;
//            }
//            // any Any Serializable path
//            n => {
//                let name = if n == "System.Net.HttpStatusCode" {
//                    format_ident!("u16")
//                } else if n == r#""gone""# {
//                    format_ident!("Gone")
//                } else if n.starts_with('"') && n.ends_with('"') {
//                    // TODO
//                    format_ident!("{}", n[1..(n.len() - 1)])
//                } else {
//                    format_ident!("{}", n)
//                };
//                tokens.extend(quote! {
//                    #name
//                });
//                return;
//            }
//        }
//    }
//}

// impl Type {
//    fn function(&self) -> TokenStream { todo!() }

//    fn array(&self) -> TokenStream {
//        let t = self.templates.iter().next().unwrap();
//        quote! {
//            Vec<#t>
//        }
//    }

//    fn map(&self) -> TokenStream {
//        let fst = self.templates.iter().next().unwrap();
//        let snd = self.templates.iter().next().unwrap();
//        quote! {
//            Map<#fst, #snd>
//        }
//    }

//    fn r#enum(&self) -> TokenStream {
//        let mut entries = self.union.iter().filter(|u| u.name != "null");
//        let num = entries.clone().count();
//        match num {
//            0 => unreachable!(),
//            1 => {
//                let first = entries.next().unwrap();
//                quote! { #first }
//            }
//            _ => {
//                quote! {
//                    NotImplementedYet
//                }
//            }
//        }
//    }

//    // TODO: recursive
//    fn declare(&self, hint: &str) -> Option<TokenStream> {
//        if !self.properties.is_empty() && self.name != "function" {
//            let name = format_ident!("NotImplementedYet{}", hint);
//            let required = self
//                .properties
//                .iter()
//                .filter(|a| a.required)
//                .map(|a| a.with_colon());
//            let opts = self
//                .properties
//                .iter()
//                .filter(|a| !a.required)
//                .map(|a| a.with_colon_option());
//            let all = required.chain(opts);
//            let nested = self
//                .properties
//                .iter()
//                .map(|a| a.ty.declare(&a.name))
//                .fold(quote! {}, |mut a, b| {
//                    a.extend(b);
//                    a
//                });
//            return Some(quote! {
//                struct #name {
//                    #(#all),*
//                }
//                #nested
//            });
//        } else if !self.union.is_empty() {
//            let name = format_ident!("NotImplementedYet{}", hint);
//            let not_null = self.union.iter().filter(|u| u.name != "null");
//            if not_null.clone().count() <= 1 {
//                return None;
//            }
//            let nested = not_null
//                .clone()
//                .map(|t| t.declare(""))
//                .fold(quote! {}, |mut a, b| {
//                    a.extend(b);
//                    a
//                });
//            let xs = not_null.map(|t| {
//                quote! { NotImplementedYet(#t) }
//            });
//            return Some(quote! {
//                enum #name {
//                    #(#xs),*
//                }
//                #nested
//            });
//        }
//        None
//    }
//}

// impl Arg {
//    fn with_colon(&self) -> TokenStream {
//        let name = self.name();
//        let ty = &self.ty;
//        let comment = &self.comment;
//        quote! {
//            #[doc = #comment]
//            #name: #ty
//        }
//    }

//    fn with_colon_option(&self) -> TokenStream {
//        let name = self.name();
//        let ty = &self.ty;
//        let comment = &self.comment;
//        quote! {
//            #[doc = #comment]
//            #name: Option<#ty>
//        }
//    }

//    fn name(&self) -> Ident { format_ident!("{}", escape(&self.name.to_snake())) }
//}
