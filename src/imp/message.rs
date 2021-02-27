use serde::{Deserialize, Serialize};
use serde_json::{map::Map, value::Value};
use strong::*;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub(crate) struct Request<'a, 'b> {
    #[serde(default)]
    pub(crate) id: Option<i32>,
    pub(crate) guid: Option<&'a Strong<Guid>>,
    #[serde(default)]
    pub(crate) method: Option<&'b Strong<Method>>,
    #[serde(default)]
    pub(crate) params: Option<Map<String, Value>>,
    #[serde(default)]
    pub(crate) result: Value
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Result(ResponseResult),
    Initial(ResponseInitial)
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ResponseResult {
    pub(crate) id: i32,
    #[serde(flatten)]
    pub(crate) body: ResponseResultBody
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ResponseResultBody {
    Success(Value),
    Error(Error)
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ResponseInitial {
    pub(crate) guid: StrongBuf<Guid>,
    pub(crate) method: StrongBuf<Method>,
    pub(crate) params: Map<String, Value>
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub(crate) fn is_create(s: &Strong<Self>) -> bool { s.as_str() == "__create__" }
    pub(crate) fn is_dispose(s: &Strong<Self>) -> bool { s.as_str() == "__dispose__" }
}
