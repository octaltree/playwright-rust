use crate::{
    api::{Frame, Response},
    imp::{core::*, prelude::*, request::Request as Impl}
};

pub struct Request {
    inner: Weak<Impl>
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
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

    pub async fn redirected_to(&self) -> Result<Option<Request>, Error> {
        Ok(upgrade(&self.inner)?.redirected_to().map(Request::new))
    }

    pub async fn response(&self) -> Result<Option<Response>, Arc<Error>> {
        Ok(upgrade(&self.inner)?.response().await?.map(Response::new))
    }
}
