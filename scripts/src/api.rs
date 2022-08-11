use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Api(pub Vec<Interface>);

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub name: String,
    // langs
    //#[serde(default)]
    // pub comment: String,
    pub spec: Vec<SpecNode>,
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
    //#[serde(default)]
    // pub comment: String,
    pub spec: Vec<SpecNode>
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
    // experimental
    // paramOrOption
    pub since: String,
    pub overload_index: usize,
    // pub comment: String,
    #[serde(default)]
    pub spec: Vec<SpecNode>,
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SpecNode {
    Text {
        text: String
    },
    Code {
        #[serde(rename = "codeLang")]
        lang: String,
        lines: Vec<String>
    },
    Li {
        #[serde(rename = "liType")]
        li_type: LiType,
        text: String
    },
    Note {
        #[serde(rename = "noteType")]
        note_type: NoteType,
        text: String
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum LiType {
    Bullet,
    Ordinal
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum NoteType {
    #[serde(rename = "caution Discouraged")]
    CautionDiscouraged,
    Caution,
    Note,
    Warning
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
            dest.push(t);
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

    #[test]
    fn parse() {
        let xs:Vec<SpecNode> = serde_json::from_str(r#"
            [{
            "type": "text",
            "text": "Captures the current state of the accessibility tree. The returned object represents the root accessible node of the page."
          },
          {
            "type": "note",
            "noteType": "note",
            "text": "The Chromium accessibility tree contains nodes that go unused on most platforms and by most screen readers. Playwright will discard them as well for an easier to process tree, unless `interestingOnly` is set to `false`."
          },
          {
            "type": "text",
            "text": "An example of dumping the entire accessibility tree:"
          }]
            "#).unwrap();
    }
}
