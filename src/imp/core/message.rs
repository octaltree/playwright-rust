use serde::{Deserialize, Serialize};
use serde_json::{map::Map, value::Value};
use strong::*;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub(crate) struct Request<'a, 'b> {
    #[serde(default)]
    pub(crate) id: i32,
    pub(crate) guid: Option<&'a S<Guid>>,
    #[serde(default)]
    pub(crate) method: Option<&'b S<Method>>,
    #[serde(default)]
    pub(crate) params: Map<String, Value>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub(crate) enum Response {
    Result(ResponseResult),
    Initial(ResponseInitial)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ResponseResult {
    pub(crate) id: i32,
    #[serde(flatten)]
    pub(crate) body: ResponseResultBody
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ResponseResultBody {
    Success(Value),
    Error(Error)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct ResponseInitial {
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
pub(crate) struct Error {
    pub(crate) name: String,
    pub(crate) message: String,
    pub(crate) stack: String
}

pub(crate) enum Guid {}

impl Validator for Guid {
    type Err = std::convert::Infallible;
}

pub(crate) enum Method {}

#[derive(Error, Debug)]
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
