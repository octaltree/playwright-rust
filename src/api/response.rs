use crate::{
    api::Request,
    imp::{core::*, prelude::*, response::Response as Impl}
};

pub struct Response {
    inner: Weak<Impl>
}

impl Response {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().into()) }
    pub fn status(&self) -> Result<i32, Error> { Ok(upgrade(&self.inner)?.status()) }
    pub fn status_text(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.status_text().into())
    }

    pub fn ok(&self) -> Result<bool, Error> { Ok(upgrade(&self.inner)?.ok()) }

    pub fn request(&self) -> Request {
        let inner = weak_and_then(&self.inner, |rc| rc.request());
        Request::new(inner)
    }

    pub async fn finished(&mut self) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.finished().await
    }

    pub async fn body(&mut self) -> ArcResult<Vec<u8>> { upgrade(&self.inner)?.body().await }

    pub async fn text(&mut self) -> ArcResult<String> { upgrade(&self.inner)?.text().await }
}
