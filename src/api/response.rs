use crate::imp::{prelude::*, response::Response as Impl};

pub struct Response {
    inner: Weak<Impl>
}

impl Response {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }
}
