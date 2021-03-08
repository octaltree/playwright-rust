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

    pub fn post_data(&self) -> Result<Option<Vec<u8>>, Error> {
        Ok(upgrade(&self.inner)?.post_data())
    }

    pub fn post_post_as_string(&self) -> Result<Option<String>, Error> {
        Ok(upgrade(&self.inner)?.post_data_as_string())
    }

    pub fn headers(&self) -> Result<HashMap<String, String>, Error> {
        Ok(upgrade(&self.inner)?.headers().clone())
    }

    pub fn redirected_from(&self) -> Result<Option<Request>, Error> {
        Ok(upgrade(&self.inner)?.redirected_from().map(Request::new))
    }
}
