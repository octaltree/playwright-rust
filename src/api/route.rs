use crate::{
    api::Request,
    imp::{core::*, prelude::*, route::Route as Impl}
};

pub struct Route {
    inner: Weak<Impl>
}

impl Route {
    fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn request(&self) -> Request {
        let inner = weak_and_then(&self.inner, |rc| rc.request());
        Request::new(inner)
    }

    pub async fn abort(&self, err_code: Option<&str>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.abort(err_code).await
    }
}
