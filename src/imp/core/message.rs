pub(crate) mod de;
pub(crate) mod ser;

use std::fmt::Debug;
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
    pub(crate) params: Map<String, Value>,
    pub(crate) metadata: crate::protocol::generated::Metadata
}

impl Default for crate::protocol::generated::Metadata {
    fn default() -> Self {
        Self {
            api_name: None,
            stack: None,
            internal: None
        }
    }
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
    #[serde(default)]
    pub(crate) params: Map<String, Value>
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CreateParams {
    #[serde(rename = "type")]
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    #[serde(default)]
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

#[derive(Debug, Deserialize, Serialize)]
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
    first_object(m)
}

pub(crate) fn first_object(m: &Map<String, Value>) -> Option<&Value> {
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
    let s = first(v)
        .ok_or(Error::InvalidParams)?
        .as_str()
        .ok_or(Error::InvalidParams)?;
    Ok(s)
}

pub(crate) fn maybe_only_str(v: &Value) -> Result<Option<&str>, Error> {
    let s = match first(v) {
        Some(s) => s.as_str().ok_or(Error::InvalidParams)?,
        None => return Ok(None)
    };
    Ok(Some(s))
}

pub(crate) fn _guid(v: &Value) -> Option<&S<Guid>> {
    let m: &Map<String, Value> = v.as_object()?;
    let v: &Value = m.get("guid")?;
    let s: &str = v.as_str()?;
    S::validate(s).ok()
}

/// If {"guid": str} then str
pub(crate) fn guid_from_params(v: &Value) -> Result<&S<Guid>, Error> {
    _guid(v).ok_or_else(|| Error::GuidNotFound(v.clone()))
}


#[derive(Debug, Serialize)]
pub(crate) struct Argument {
    pub(crate) value: Map<String, Value>,
    pub(crate) handles: Vec<OnlyGuid>
}

#[derive(Debug, Deserialize)]
pub struct DateTime {
    d: String
}

mod datetime {
    use super::*;
    #[cfg(feature = "chrono")]
    use chrono::Utc;
    use serde::{ser, ser::SerializeStruct};
    use std::convert::TryFrom;

    #[cfg(feature = "chrono")]
    impl From<chrono::DateTime<Utc>> for DateTime {
        fn from(c: chrono::DateTime<Utc>) -> DateTime { Self { d: c.to_rfc3339() } }
    }

    #[cfg(feature = "chrono")]
    impl TryFrom<DateTime> for chrono::DateTime<Utc> {
        type Error = chrono::format::ParseError;

        fn try_from(d: DateTime) -> Result<chrono::DateTime<Utc>, Self::Error> {
            let f = chrono::DateTime::parse_from_rfc3339(&d.d)?;
            Ok(f.with_timezone(&chrono::Utc))
        }
    }

    impl ser::Serialize for DateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer
        {
            let mut s = serializer.serialize_struct("e7ee19d3-64cb-4286-8762-6dd8ab78eb89", 1)?;
            s.serialize_field("d", &self.d)?;
            s.end()
        }
    }
}
