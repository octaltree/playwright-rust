use crate::{
    api::{Frame, Request},
    imp::{core::*, prelude::*, response::Response as Impl, utils::Header}
};

#[derive(Debug, Clone)]
pub struct Response {
    inner: Weak<Impl>
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl Response {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().into()) }
    /// Contains the status code of the response (e.g., 200 for a success).
    pub fn status(&self) -> Result<i32, Error> { Ok(upgrade(&self.inner)?.status()) }
    /// Contains the status text of the response (e.g. usually an "OK" for a success).
    pub fn status_text(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.status_text().into())
    }

    /// Contains a boolean stating whether the response was successful (status in the range 200-299) or not.
    pub fn ok(&self) -> Result<bool, Error> { Ok(upgrade(&self.inner)?.ok()) }

    pub fn request(&self) -> Request {
        let inner = weak_and_then(&self.inner, |rc| rc.request());
        Request::new(inner)
    }

    /// Waits for this response to finish, returns failure error if request failed.
    pub async fn finished(&self) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.finished().await
    }

    pub async fn body(&self) -> ArcResult<Vec<u8>> { upgrade(&self.inner)?.body().await }

    /// Returns the text representation of response body.
    pub async fn text(&self) -> ArcResult<String> { upgrade(&self.inner)?.text().await }

    /// Returns the object with HTTP headers associated with the response. All header names are lower-case.
    pub async fn headers(&self) -> ArcResult<Vec<Header>> { upgrade(&self.inner)?.headers().await }

    /// Shortcut for [`Response::request`]'s  [`Request::frame`]
    pub fn frame(&self) -> Frame { self.request().frame() }
}
