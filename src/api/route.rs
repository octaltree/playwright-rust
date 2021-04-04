use crate::{
    api::{Header, Request},
    imp::{
        core::*,
        prelude::*,
        route::{ContinueArgs, FulfillArgs, Route as Impl}
    }
};

pub struct Route {
    inner: Weak<Impl>
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
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

    pub async fn fulfill_builder<'a>(
        &self,
        body: &'a str,
        is_base64: bool
    ) -> FulfillBuilder<'a, '_> {
        FulfillBuilder::new(self.inner.clone(), body, is_base64)
    }

    pub async fn continue_builder(&self) -> ContinueBuilder<'_, '_, '_> {
        ContinueBuilder::new(self.inner.clone())
    }
}

pub struct FulfillBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: FulfillArgs<'a, 'b>
}

impl<'a, 'b> FulfillBuilder<'a, 'b> {
    pub(crate) fn new(inner: Weak<Impl>, body: &'a str, is_base64: bool) -> Self {
        let args = FulfillArgs::new(body, is_base64);
        Self { inner, args }
    }

    pub async fn fulfill(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.fulfill(args).await
    }

    pub fn headers<T>(mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>
    {
        self.args.headers = Some(x.into_iter().map(Header::from).collect());
        self
    }

    optional_setter!(
        status, i32;
        content_type, &'b str);

    pub fn clear_headers(mut self) -> Self {
        self.args.headers = None;
        self
    }
}

pub struct ContinueBuilder<'a, 'b, 'c> {
    inner: Weak<Impl>,
    args: ContinueArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> ContinueBuilder<'a, 'b, 'c> {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = ContinueArgs::default();
        Self { inner, args }
    }

    pub async fn r#continue(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.r#continue(args).await
    }

    pub fn headers<T>(mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>
    {
        self.args.headers = Some(x.into_iter().map(Header::from).collect());
        self
    }

    optional_setter!(
        url, &'a str;
        method, &'b str;
        post_data, &'c str);

    pub fn clear_headers(mut self) -> Self {
        self.args.headers = None;
        self
    }
}
