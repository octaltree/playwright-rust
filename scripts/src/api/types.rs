use super::{Arg, Interface, Kind, Member, Type};
use crate::utils;
use case::CaseExt;
use itertools::Itertools;
use std::{
    borrow::Cow,
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
        name: Cow<'static, str>,
        reference: bool
    }
}

#[derive(Debug, Clone)]
pub struct Method<'a> {
    pub orig: &'a Member,
    pub args: Vec<(String, Rc<Model>)>,
    pub builder: Option<Rc<Model>>,
    pub ty: Rc<Model>
}

#[derive(Debug, Clone)]
pub struct Event<'a> {
    pub orig: Vec<&'a Member>,
    pub model: Rc<Model>,
    pub which: Rc<Model>
}

impl Model {
    pub fn maybe_option(&self) -> Option<&Model> {
        match self {
            Model::Option(x) => Some(x),
            _ => None
        }
    }
}

impl<'a> Event<'a> {
    fn new_dummy() -> Self {
        Self {
            orig: vec![],
            model: Rc::new(Model::Known {
                name: Cow::Borrowed(""),
                reference: false
            }),
            which: Rc::new(Model::Known {
                name: Cow::Borrowed(""),
                reference: false
            })
        }
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

    pub fn has_reference(&self) -> bool {
        match self {
            Model::Struct { has_reference, .. } => *has_reference,
            Model::Enum { has_reference, .. } => *has_reference,
            Model::Option(x) => x.has_reference(),
            Model::Vec(x) => x.has_reference(),
            Model::Map(k, v) => k.has_reference() || v.has_reference(),
            Model::Known { reference, .. } => *reference
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

pub fn collect_types<'a>(x: &'a Interface) -> (Vec<Rc<Model>>, Vec<Method<'a>>, Option<Event<'a>>) {
    let mut top = Vec::new();
    let mut methods = Vec::new();
    let mut event = Event::new_dummy();
    let mut event_types = Vec::new();
    for member in &x.members {
        let mut args = Vec::new();
        let mut builder_properties = Vec::new();
        let needs_builder = needs_builder(member);
        let mut handle_arg = |arg: &Arg| {
            let ty = declare_ty(vec![&member.name, &arg.name], &arg.ty, true);
            match (needs_builder, arg.required) {
                (true, true) => {
                    builder_properties.push((utils::loud_to_snake(&arg.name), ty.clone()));
                }
                (true, false) => {
                    builder_properties.push((
                        utils::loud_to_snake(&arg.name),
                        Rc::new(Model::Option(ty.clone()))
                    ));
                }
                (false, true) => {
                    args.push((arg.name.clone(), ty.clone()));
                }
                (false, false) => ()
            }
            top.push(ty);
        };
        for arg in &member.args {
            if is_action_csharp(arg) {
                continue;
            }
            if arg.name == "options" {
                for p in &arg.ty.properties {
                    handle_arg(p);
                }
            } else {
                handle_arg(arg);
            }
        }
        let builder = if needs_builder {
            let has_reference = builder_properties.iter().any(|(_, p)| p.has_reference());
            let b = Rc::new(Model::Struct {
                name: format!(
                    "{}Builder",
                    utils::loud_to_camel(&member.name.to_camel().replace("#", ""))
                ),
                orig: Type {
                    name: "".into(),
                    expression: None,
                    properties: vec![],
                    templates: vec![],
                    union: vec![]
                },
                fields: builder_properties,
                has_reference
            });
            top.push(b.clone());
            Some(b)
        } else {
            None
        };
        let ty = declare_ty(vec![&member.name], &member.ty, false);
        top.push(ty.clone());
        match member.kind {
            Kind::Method | Kind::Property => {
                methods.push(Method {
                    orig: member,
                    args,
                    builder,
                    ty
                });
            }
            Kind::Event => {
                event.orig.push(member);
                event_types.push(ty);
            }
        }
    }
    event_model(&mut event, event_types);
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
            Model::Known { .. } => {}
        }
    }
    let types = all.into_iter().filter(|t| t.orig().is_some()).collect();
    let event = if event.orig.is_empty() {
        None
    } else {
        Some(event)
    };
    (types, methods, event)
}

fn maybe_builder(member: &Member) -> Option<Rc<Model>> {
    if !needs_builder(member) {
        return None;
    }
    let properties = member
        .args
        .iter()
        .flat_map(|arg| {
            if arg.name == "options" {
                arg.ty.properties.clone()
            } else {
                vec![arg.clone()]
            }
        })
        .filter(|arg| !is_action_csharp(arg))
        .collect();
    Some(declare_ty(
        vec![&member.name, "builder"],
        &Type {
            name: "builder".into(),
            expression: None,
            properties,
            templates: vec![],
            union: vec![]
        },
        true
    ))
}

/// has two or more optional values
pub fn needs_builder(member: &Member) -> bool {
    let args = &member.args;
    let mut xs = args.iter().filter(|a| !a.required).chain(
        args.iter()
            .filter(|a| a.name == "options" && !a.ty.properties.is_empty())
            .flat_map(|a| a.ty.properties.iter())
    );
    xs.next().and(xs.next()).is_some()
}

fn declare_ty<'a>(scope: Vec<&'a str>, ty: &'a Type, allow_borrow: bool) -> Rc<Model> {
    if ty.union.is_empty() {
        match (ty.properties.is_empty(), ty.templates.is_empty()) {
            (true, true) => Rc::new(match &*ty.name {
                "binary" => Model::Known {
                    name: Cow::Borrowed("binary"),
                    reference: allow_borrow
                },
                "json" => Model::Known {
                    name: Cow::Borrowed("json"),
                    reference: allow_borrow
                },
                "number" => Model::Known {
                    name: Cow::Borrowed("number"),
                    reference: false
                },
                "float" => Model::Known {
                    name: Cow::Borrowed("float"),
                    reference: false
                },
                "string" => Model::Known {
                    name: Cow::Borrowed("string"),
                    reference: allow_borrow
                },
                "boolean" => Model::Known {
                    name: Cow::Borrowed("boolean"),
                    reference: false
                },
                "void" => Model::Known {
                    name: Cow::Borrowed("void"),
                    reference: false
                },
                x => Model::Known {
                    name: Cow::Owned(utils::loud_to_camel(x)),
                    reference: false
                }
            }),
            (false, true) => {
                let name = scope
                    .iter()
                    .map(|s| utils::loud_to_camel(&s.to_camel().replace("#", "")))
                    .join("");
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
                                t
                            } else {
                                Rc::new(Model::Option(t))
                            }
                        )
                    })
                    .collect::<Vec<_>>();
                let has_reference = fields
                    .iter()
                    .map(|(_, m)| m.has_reference())
                    .fold(false, |a, b| a || b);
                Rc::new(Model::Struct {
                    name,
                    orig: ty.clone(),
                    fields,
                    has_reference
                })
            }
            (true, false) if ty.name == "Func" => unreachable!("{:?}", &ty),
            (true, false) if ty.name == "Array" => {
                assert_eq!(ty.templates.len(), 1);
                let t = &ty.templates[0];
                Rc::new(Model::Vec(declare_ty(scope, t, allow_borrow)))
            }
            (true, false) => {
                let option = ty.expression.as_deref();
                assert!(
                    option
                        == Some("[IReadOnlyDictionary<string, BrowserNewContextOptions>]")
                        || option == Some("[Map]<[string], [JSHandle]>")
                        || option == Some("[Map<string, Object>]")
                        || option == Some("[Object]<[string], [string]>")
                        || option
                            == Some("[Object]<[string], [string]|[float]|[boolean]>")
                        || option == Some("[Object]<[string], [Serializable]>")
                        || option == Some("[Object]<[string], [any]>"),
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
                if t.name.contains("\"") {
                    return declare_enum(scope, ty, allow_borrow);
                }
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
        a.push_str(&utils::loud_to_camel(&b.replace("#", "").to_camel()));
        a
    });
    let variants = ty
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
        .collect::<Vec<_>>();
    let has_reference = variants
        .iter()
        .map(|v| {
            v.data
                .as_ref()
                .map(|data| data.has_reference())
                .unwrap_or_default()
        })
        .fold(false, |a, b| a || b);
    Rc::new(Model::Enum {
        name,
        orig: ty.clone(),
        variants,
        has_reference
    })
}

fn event_model(event: &mut Event, models: Vec<Rc<Model>>) {
    event.model = Rc::new(Model::Enum {
        name: "Event".into(),
        orig: Type {
            name: "Event".into(),
            expression: None,
            properties: vec![],
            templates: vec![],
            union: event.orig.iter().map(|member| member.ty.clone()).collect()
        },
        variants: event
            .orig
            .iter()
            .zip(models.iter())
            .map(|(member, model)| {
                let data = match &**model {
                    Model::Known {
                        name: Cow::Borrowed("void"),
                        ..
                    } => None,
                    _ => Some(model.clone())
                };
                Variant {
                    label: utils::loud_to_camel(&member.name.to_camel().replace("#", "")),
                    orig: member.name.clone(),
                    data
                }
            })
            .collect(),
        has_reference: false
    });
    event.which = Rc::new(Model::Enum {
        name: "EventType".into(),
        orig: Type {
            name: "EventType".into(),
            expression: None,
            properties: vec![],
            templates: vec![],
            union: event.orig.iter().map(|member| member.ty.clone()).collect()
        },
        variants: event
            .orig
            .iter()
            .zip(models.iter())
            .map(|(member, model)| Variant {
                label: utils::loud_to_camel(&member.name.to_camel().replace("#", "")),
                orig: member.name.clone(),
                data: None
            })
            .collect(),
        has_reference: false
    });
}
