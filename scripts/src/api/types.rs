use super::{Arg, Interface, Type};
use crate::utils;
use case::CaseExt;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    Struct {
        name: String,
        orig: Type,
        fields: Vec<(String, Rc<Model>)>,
        /// memoization
        has_reference: bool
    },
    Enum {
        name: String,
        orig: Type,
        variants: Vec<Variant>,
        has_reference: bool
    },
    Option(Rc<Model>),
    Vec(Rc<Model>),
    Map(Rc<Model>, Rc<Model>),
    Known {
        name: &'static str,
        reference: bool
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub label: String,
    pub orig: String,
    pub data: Option<Rc<Model>>
}

impl Model {
    fn orig(&self) -> Option<&Type> {
        match self {
            Self::Struct { orig, .. } => Some(orig),
            Self::Enum { orig, .. } => Some(orig),
            _ => None
        }
    }

    fn has_reference(&self) -> bool {
        match self {
            Model::Struct { has_reference, .. } => *has_reference,
            Model::Enum { has_reference, .. } => *has_reference,
            Model::Option(x) => x.has_reference(),
            Model::Vec(x) => x.has_reference(),
            Model::Map(k, v) => k.has_reference() || v.has_reference(),
            Model::Known { name, reference } => *reference
        }
    }
}

pub fn is_action_csharp(a: &Arg) -> bool {
    let for_csharp = if let Some(only) = &a.langs.only {
        only.len() == 1 && only[0] == "csharp"
    } else {
        false
    };
    a.name == "action" && for_csharp
}

pub fn collect_types(x: &Interface) -> Vec<Rc<Model>> {
    let mut top = Vec::new();
    for member in &x.members {
        for arg in &member.args {
            if is_action_csharp(arg) {
                continue;
            }
            top.push(declare_ty(vec![&member.name, &arg.name], &arg.ty, true));
        }
        top.push(declare_ty(vec![&member.name], &member.ty, false));
    }
    let mut que: VecDeque<_> = top.into();
    let mut all = Vec::new();
    while let Some(x) = que.pop_front() {
        all.push(x.clone());
        match &*x {
            Model::Struct { fields, .. } => {
                for (_, m) in fields {
                    que.push_back(m.clone());
                }
            }
            Model::Enum { variants, .. } => {
                for Variant { data: m, .. } in variants {
                    if let Some(m) = m {
                        que.push_back(m.clone());
                    }
                }
            }
            Model::Vec(x) | Model::Option(x) => que.push_back(x.clone()),
            Model::Map(k, x) => {
                que.push_back(k.clone());
                que.push_back(x.clone());
            }
            Model::Known { name, reference } => {}
        }
    }
    all.into_iter()
        .filter(|t| t.orig().is_some())
        .group_by(|t| t.orig().unwrap().clone())
        .into_iter()
        .map(|(_, group)| {
            let xs = group.collect::<Vec<_>>();
            if xs.len() == 1 {
                xs[0].clone()
            } else {
                match &*xs[0] {
                    Model::Struct {
                        name: _,
                        orig,
                        fields,
                        has_reference
                    } => Rc::new(Model::Struct {
                        name: orig.name.to_string(),
                        orig: orig.clone(),
                        fields: fields.clone(),
                        has_reference: *has_reference
                    }),
                    Model::Enum {
                        name: _,
                        orig,
                        variants,
                        has_reference
                    } => Rc::new(Model::Enum {
                        name: orig.name.to_string(),
                        orig: orig.clone(),
                        variants: variants.clone(),
                        has_reference: *has_reference
                    }),
                    _ => unreachable!()
                }
            }
        })
        .collect()
}

fn declare_ty<'a>(scope: Vec<&'a str>, ty: &'a Type, allow_borrow: bool) -> Rc<Model> {
    if ty.union.is_empty() {
        match (ty.properties.is_empty(), ty.templates.is_empty()) {
            (true, true) => Rc::new(match &*ty.name {
                "binary" => Model::Array {
                    data: Rc::new(Model::Struct {
                        name: "u8".to_string(),
                        has_reference: false,
                        orig: todo!(),
                        fields: vec![]
                    }),
                    reference: allow_borrow
                },
                "number" => quote!(serde_json::Number),
                "float" => quote!(f64),
                "json" if allow_borrow => quote!(&'a str),
                "json" => quote!(String),
                "string" if allow_borrow => quote!(&'a str),
                "string" => quote!(String),
                "boolean" => quote!(bool),
                "void" => quote!(()),
                x => {
                    let ident = format_ident!("{}", utils::loud_to_camel(x));
                    quote!(#ident)
                }
            }),
            (false, true) => {
                let name = scope
                    .iter()
                    .map(|s| utils::loud_to_camel(&s.to_camel().replace("#", "")))
                    .join("\n");

                let fields = ty
                    .properties
                    .iter()
                    .map(|p| {
                        let field_name = utils::loud_to_snake(&p.name);
                        let mut tmp = scope.clone();
                        tmp.push(&p.name);
                        let t = declare_ty(tmp, &p.ty, allow_borrow);
                        (
                            field_name,
                            if p.required {
                                Rc::new(Model::Option(t))
                            } else {
                                t
                            }
                        )
                    })
                    .collect();
                Rc::new(Model::Struct {
                    name,
                    orig: ty.clone(),
                    fields
                })
            }
            (true, false) if ty.name == "Func" => unreachable!(),
            (true, false) if ty.name == "Array" => {
                assert_eq!(ty.templates.len(), 1);
                let t = &ty.templates[0];
                Rc::new(Model::Vec(declare_ty(scope, t, allow_borrow)))
            }
            (true, false) => {
                assert!(
                    ty.expression.as_deref()
                        == Some("[IReadOnlyDictionary<string, BrowserNewContextOptions>]")
                        || ty.expression.as_deref() == Some("[Map]<[string], [JSHandle]>")
                        || ty.expression.as_deref() == Some("[Object]<[string], [string]>"),
                    "{:?}",
                    &ty
                );
                assert_eq!(ty.templates.len(), 2);
                Rc::new(Model::Map(
                    declare_ty(scope.clone(), &ty.templates[0], allow_borrow),
                    declare_ty(scope, &ty.templates[1], allow_borrow)
                ))
            }
            (false, false) => {
                assert_eq!(
                    ty.expression.as_deref(),
                    Some("[Object]<[string], [string]|[float]|[boolean]|[ReadStream]|[Object]>")
                );
                assert_eq!(ty.templates.len(), 2);
                Rc::new(Model::Map(
                    declare_ty(scope.clone(), &ty.templates[0], allow_borrow),
                    declare_ty(scope, &ty.templates[1], allow_borrow)
                ))
            }
        }
    } else {
        let variants = ty.union.iter().filter(|t| t.name != "null");
        let num_variants = variants.clone().count();
        match num_variants {
            0 => unreachable!(),
            1 => {
                let mut vs = variants;
                let t = vs.next().unwrap();
                declare_ty(scope, t, allow_borrow)
            }
            _ => declare_enum(scope, ty, allow_borrow)
        }
    }
}

fn declare_enum<'a>(scope: Vec<&'a str>, ty: &'a Type, allow_borrow: bool) -> Rc<Model> {
    assert_eq!(ty.properties, &[]);
    assert_eq!(ty.templates, &[]);
    let name = scope.iter().fold(String::new(), |mut a, b| {
        a.push_str(&utils::loud_to_camel(&b.to_camel()));
        a
    });
    Rc::new(Model::Enum {
        name,
        orig: ty.clone(),
        variants: ty
            .union
            .iter()
            .filter(|t| t.name != "null")
            .map(|t| {
                if t.name.contains("\"") {
                    let name = t.name.replace("\"", "");
                    let label = utils::kebab_to_camel(&name);
                    Variant {
                        orig: name,
                        label,
                        data: None
                    }
                } else {
                    let name = &t.name;
                    let label = utils::kebab_to_camel(&name);
                    let mut tmp = scope.clone();
                    tmp.push(name);
                    Variant {
                        orig: name.to_string(),
                        label,
                        data: Some(declare_ty(tmp, t, allow_borrow))
                    }
                }
            })
            .collect()
    })
}
