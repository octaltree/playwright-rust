use serde::{Deserialize, Deserializer};
use serde_json::{map::Map, value::Value};
use strong::*;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub(crate) struct Request<'a, 'b> {
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
pub(crate) enum Response {
    Result(ResponseResult),
    Initial(ResponseInitial)
}

#[derive(Debug, Clone)]
pub struct ResponseResult {
    pub(crate) id: i32,
    pub(crate) body: Result<Value, Error>
}

impl<'de> Deserialize<'de> for ResponseResult {
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
pub(crate) struct ErrorWrap {
    error: Error
}

#[derive(Debug, Deserialize, Serialize, Clone, thiserror::Error)]
#[error("{name} {message:?}")]
pub struct Error {
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
