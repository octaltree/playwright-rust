use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    let protocol: Protocol = serde_yaml::from_reader(std::io::stdin()).unwrap();
    let t = protocol.into_token_stream();
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct Protocol(HashMap<String, Node>);

#[derive(Debug, PartialEq)]
enum Node {
    Enum(Enum),
    Object(Object),
    Mixin(Mixin),
    Interface(Interface)
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Debug, Deserialize, PartialEq)]
        struct A {
            r#type: String,
            commands: Option<Commands>,
            events: Option<Events>,
            extends: Option<String>,
            initializer: Option<Properties>,
            literals: Option<Vec<String>>,
            properties: Option<Properties>
        }
        let a = A::deserialize(deserializer)?;
        Ok(match &*a.r#type {
            "object" => Self::Object(Object {
                properties: a.properties.unwrap()
            }),
            "mixin" => Self::Mixin(Mixin {
                properties: a.properties.unwrap()
            }),
            "enum" => Self::Enum(Enum {
                literals: a.literals.unwrap()
            }),
            "interface" => {
                let A {
                    commands,
                    events,
                    extends,
                    initializer,
                    ..
                } = a;
                Self::Interface(Interface {
                    commands,
                    events,
                    extends,
                    initializer
                })
            }
            _ => unreachable!()
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Enum {
    literals: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Interface {
    commands: Option<Commands>,
    events: Option<Events>,
    extends: Option<String>,
    initializer: Option<Properties>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Mixin {
    properties: Properties
}

/// ex. {"close": null}
type Commands = HashMap<String, Option<Command>>;
type Events = HashMap<String, Option<Event>>;
type Properties = HashMap<String, Type>;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(transparent)]
struct Object {
    properties: Properties
}

#[derive(Debug, Deserialize, PartialEq)]
struct Command {
    parameters: Option<Properties>,
    returns: Option<Properties>,
    // True as String
    #[serde(default)]
    experimental: Option<String>,
    tracing: Option<Tracing>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Tracing {
    snapshot: bool,
    pause_before_input: Option<bool>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Event {
    parameters: Properties
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
enum Type {
    Name(String),
    Items {
        r#type: String,
        items: Box<Type>
    },
    Literals {
        r#type: String,
        literals: Vec<String>
    },
    Properties {
        r#type: String,
        properties: Properties
    }
}

impl ToTokens for Protocol {
    fn to_tokens(&self, tokens: &mut TokenStream) { todo!() }
}

fn node((name, node): (&str, &Node)) { todo!() }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn all_types() {
        let s = fs::read_to_string("../src/protocol/protocol.yml").unwrap();
        let s = s.replace("null", r#""null""#);
        let protocol: Protocol = serde_yaml::from_str(&s).unwrap();
        let mut types = Vec::new();
        fn add<'a>(dest: &mut Vec<&'a Type>, t: &'a Type) {
            match t {
                Type::Name(_) => dest.push(t),
                Type::Items { items, .. } => add(dest, &items),
                Type::Literals { .. } => {}
                Type::Properties { properties, .. } => {
                    for (_, t) in properties.iter() {
                        add(dest, t);
                    }
                }
            }
        }
        for (_, node) in protocol.0.iter() {
            match node {
                Node::Object(Object { properties }) | Node::Mixin(Mixin { properties }) => {
                    for (_, t) in properties {
                        add(&mut types, t);
                    }
                }
                Node::Interface(Interface {
                    commands,
                    events,
                    extends: _,
                    initializer
                }) => {
                    fn append<'a>(dest: &mut Vec<&'a Type>, props: &'a Option<Properties>) {
                        for (_, t) in props.iter().flat_map(|m| m.iter()) {
                            add(dest, t);
                        }
                    }
                    for (_, c) in commands.iter().flat_map(|m| m.iter()) {
                        let c = if c.is_none() {
                            continue;
                        } else {
                            c.as_ref().unwrap()
                        };
                        append(&mut types, &c.parameters);
                        append(&mut types, &c.returns);
                    }
                    for (_, e) in events.iter().flat_map(|m| m.iter()) {
                        let e = if e.is_none() {
                            continue;
                        } else {
                            e.as_ref().unwrap()
                        };
                        for (_, t) in e.parameters.iter() {
                            add(&mut types, t);
                        }
                    }
                    append(&mut types, initializer);
                }
                _ => {}
            }
        }
        println!("{}", serde_json::to_string(&types).unwrap());
    }
}
