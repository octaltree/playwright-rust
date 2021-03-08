use crate::imp::core::Error;
use serde::{Deserialize, Deserializer};
use serde_json::{map::Map, value::Value};
use strong::*;

#[derive(Debug, Serialize)]
pub(crate) struct Req<'a, 'b> {
    #[serde(default)]
    pub(crate) id: i32,
    pub(crate) guid: &'a S<Guid>,
    #[serde(default)]
    pub(crate) method: &'b S<Method>,
    #[serde(default)]
    pub(crate) params: Map<String, Value>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum Res {
    Result(ResResult),
    Initial(ResInitial)
}

#[derive(Debug, Clone)]
pub(crate) struct ResResult {
    pub(crate) id: i32,
    pub(crate) body: Result<Value, ErrorMessage>
}

impl<'de> Deserialize<'de> for ResResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct ResponseResultImpl {
            id: i32,
            result: Option<Value>,
            error: Option<ErrorWrap>
        }
        let ResponseResultImpl { id, result, error } =
            ResponseResultImpl::deserialize(deserializer)?;
        if let Some(ErrorWrap { error }) = error {
            Ok(Self {
                id,
                body: Err(error)
            })
        } else if let Some(x) = result {
            Ok(Self { id, body: Ok(x) })
        } else {
            Ok(Self {
                id,
                body: Ok(Value::default())
            })
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ResInitial {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CreateParams {
    #[serde(rename = "type")]
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    pub(crate) initializer: Value
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ErrorWrap {
    error: ErrorMessage
}

#[derive(Debug, Deserialize, Serialize, Clone, thiserror::Error)]
#[error("{name} {message:?}")]
pub struct ErrorMessage {
    pub(crate) name: String,
    pub(crate) message: String,
    pub(crate) stack: String
}

#[derive(Debug, Deserialize)]
pub(crate) struct OnlyGuid {
    pub(crate) guid: Str<Guid>
}

pub(crate) enum Guid {}

impl Validator for Guid {
    type Err = std::convert::Infallible;
}

pub(crate) enum Method {}

#[derive(thiserror::Error, Debug)]
#[error("Method {0:?} validation error")]
pub(crate) struct MethodError(String);

impl Validator for Method {
    type Err = MethodError;

    fn validate(raw: &str) -> Result<(), Self::Err> {
        if raw.is_empty() {
            Err(MethodError(raw.to_string()))
        } else {
            Ok(())
        }
    }
}

impl Method {
    pub(crate) fn is_create(s: &S<Self>) -> bool { s.as_str() == "__create__" }
    pub(crate) fn is_dispose(s: &S<Self>) -> bool { s.as_str() == "__dispose__" }
}

pub(crate) enum ObjectType {}

impl Validator for ObjectType {
    type Err = std::convert::Infallible;
}

pub(crate) fn first(v: &Value) -> Option<&Value> {
    let m: &Map<String, Value> = v.as_object()?;
    if m.len() != 1 {
        return None;
    }
    let v: &Value = m.values().next()?;
    Some(v)
}

/// If {"<type>": {"guid": str}} then str
pub(crate) fn as_only_guid(v: &Value) -> Option<&S<Guid>> {
    // {"<type>": {"guid": str}}
    let v: &Value = first(v)?;
    // {"guid": str}
    let m: &Map<String, Value> = v.as_object()?;
    let v: &Value = m.get("guid")?;
    let s: &str = v.as_str()?;
    S::validate(s).ok()
}

pub(crate) fn only_guid(v: &Value) -> Result<&S<Guid>, Error> {
    as_only_guid(v).ok_or_else(|| Error::GuidNotFound(v.clone()))
}

pub(crate) fn only_str(v: &Value) -> Result<&str, Error> {
    let s = first(&v)
        .ok_or(Error::InvalidParams)?
        .as_str()
        .ok_or(Error::InvalidParams)?;
    Ok(s)
}

pub(crate) fn maybe_only_str(v: &Value) -> Result<Option<&str>, Error> {
    let s = match first(&v) {
        Some(s) => s.as_str().ok_or(Error::InvalidParams)?,
        None => return Ok(None)
    };
    Ok(Some(s))
}

// pub(crate) fn parse_value(v: &Value) -> Result<Value, ()> {
//    if let Value::Object(v) = v {
//        if v.contains_key("v") {
//            match  v.get("v").unwrap() {
//                Value::String("Infinity") =>
//            }
//            //}else if v.contains_key("a") { // array
//            //}else if v.contains_key("d") { // datetime
//            //}else if v.contains_key("o") { // object
//            //}else if v.contains_key("n") {
//            //    s.get("n").unwrap()
//            //}else if v.contains_key("s") {
//            //    s.get("s").unwrap()
//            //}else if v.contains_key("b") { // bytes?
//            //    v.get("b").unwrap()
//        } else {
//            v.clone()
//        }
//    } else {
//        v.clone()
//    }
//}
