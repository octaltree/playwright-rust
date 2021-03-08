use crate::{
    api::frame::Frame,
    imp::{core::*, prelude::*, request::Request as Impl}
};

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

    pub fn is_navigation_request(&self) -> Result<bool, Error> {
        Ok(upgrade(&self.inner)?.is_navigation_request())
    }

    pub fn frame(&self) -> Frame {
        let inner = weak_and_then(&self.inner, |rc| rc.frame());
        Frame::new(inner)
    }
}
