use crate::imp::{core::*, prelude::*, request::Request as Impl};

pub struct Request {
    inner: Weak<Impl>
}

impl Request {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn method(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.method().into()) }

    pub fn resource_type(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.resource_type().into())
    }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().into()) }
}
