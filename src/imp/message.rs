use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Request {}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response {}

impl Response {
    pub(crate) fn new() -> Self { Self {} }
}

pub(crate) struct Error {
    name: String,
    message: String,
    stack: String
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Method {
    #[serde(rename = "__create__")]
    Create,
    #[serde(rename = "__dispose__")]
    Dispose
}
