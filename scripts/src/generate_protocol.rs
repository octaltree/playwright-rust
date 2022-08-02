use serde::Deserialize;
use std::collections::HashMap;

fn main() {
    let protocol: Protocol = serde_yaml::from_reader(std::io::stdin()).unwrap();
    let t = "";
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

#[derive(Debug, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let x: Properties = serde_yaml::from_str("n: number?").unwrap();
        let x: Type = serde_yaml::from_str(
            r#"
      type: enum?
      literals:
      - "null"
      - undefined
      - NaN
      - Infinity
      - -Infinity
      - "-0""#
        )
        .unwrap();
        let x: HashMap<String, Node> = serde_yaml::from_str(
            r#"
    SerializedValue:
      type: object
      # Exactly one of the properties must be present.
      properties:
        n: number?
        b: boolean?
        s: string?
        v:
          type: enum?
          literals:
          - "null"
          - undefined
          - NaN
          - Infinity
          - -Infinity
          - "-0""#
        )
        .unwrap();
    }
}
