use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum Request {}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response {}

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
