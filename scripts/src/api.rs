use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Api(pub Vec<Interface>);

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub name: String,
    // langs
    // spec: Vec<SpecNode>,
    #[serde(default)]
    pub comment: String,
    pub members: Vec<Member>,
    pub extends: Option<String>
}

/// ex. {"name": "toMatchSnapshot#2", "alias": "toMatchSnapshot", "overloadIndex": 1}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub kind: Kind,
    pub name: String,
    pub alias: String,
    pub experimental: bool,
    pub since: String,
    pub overload_index: usize,
    pub required: bool,
    #[serde(rename = "async")]
    pub is_async: bool,
    pub args: Vec<Arg>,
    #[serde(rename = "type")]
    pub ty: Type,
    // langs
    // paramOrOption null
    pub deprecated: bool,
    // spec: Vec<SpecNode>,
    #[serde(default)]
    pub comment: String
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Event,
    Method,
    Property
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Type {
    pub name: String,
    #[serde(default)]
    pub expression: Option<String>,
    #[serde(default)]
    pub properties: Vec<Arg>,
    #[serde(default)]
    pub templates: Vec<Type>,
    #[serde(default)]
    pub union: Vec<Type>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub name: String,
    pub kind: ArgKind,
    pub alias: String,
    #[serde(rename = "type")]
    pub ty: Type,
    // langs
    // spec
    // experimental
    // paramOrOption
    pub since: String,
    pub overload_index: usize,
    pub comment: String,
    pub required: bool,
    pub deprecated: bool,
    #[serde(rename = "async")]
    pub is_async: bool
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ArgKind {
    Property
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn all_api_types() {
        let mut types = Vec::new();
        fn add<'a>(dest: &mut Vec<&'a Type>, t: &'a Type) {
            let Type {
                name,
                expression: _,
                properties,
                templates,
                union
            }: &Type = t;
            if !name.is_empty() {
                dest.push(t);
            }
            for arg in properties {
                add(dest, &arg.ty);
            }
            for ty in templates {
                add(dest, ty);
            }
            for ty in union {
                add(dest, ty);
            }
        }
        let s = fs::read_to_string("../src/api/api.json").unwrap();
        let api: Api = serde_json::from_str(&s).unwrap();
        for interface in &api.0 {
            for member in &interface.members {
                for arg in &member.args {
                    add(&mut types, &arg.ty);
                }
                add(&mut types, &member.ty);
            }
        }
        println!("{}", serde_json::to_string(&types).unwrap());
    }
}
