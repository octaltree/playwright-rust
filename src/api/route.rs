use crate::{
    api::{Header, Request},
    imp::{
        core::*,
        prelude::*,
        route::{ContinueArgs, FulfillArgs, Route as Impl}
    }
};

/// Whenever a network route is set up with [`method: Page.route`] or [`method: BrowserContext.route`], the `Route` object
/// allows to handle the route.
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

    /// A request to be routed.
    pub fn request(&self) -> Request {
        let inner = weak_and_then(&self.inner, |rc| rc.request());
        Request::new(inner)
    }

    /// Aborts the route's request.
    /// Optional error code. Defaults to `failed`, could be one of the following:
    /// - `'aborted'` - An operation was aborted (due to user action)
    /// - `'accessdenied'` - Permission to access a resource, other than the network, was denied
    /// - `'addressunreachable'` - The IP address is unreachable. This usually means that there is no route to the specified
    ///  host or network.
    /// - `'blockedbyclient'` - The client chose to block the request.
    /// - `'blockedbyresponse'` - The request failed because the response was delivered along with requirements which are not
    ///  met ('X-Frame-Options' and 'Content-Security-Policy' ancestor checks, for instance).
    /// - `'connectionaborted'` - A connection timed out as a result of not receiving an ACK for data sent.
    /// - `'connectionclosed'` - A connection was closed (corresponding to a TCP FIN).
    /// - `'connectionfailed'` - A connection attempt failed.
    /// - `'connectionrefused'` - A connection attempt was refused.
    /// - `'connectionreset'` - A connection was reset (corresponding to a TCP RST).
    /// - `'internetdisconnected'` - The Internet connection has been lost.
    /// - `'namenotresolved'` - The host name could not be resolved.
    /// - `'timedout'` - An operation timed out.
    /// - `'failed'` - A generic failure occurred.
    pub async fn abort(&self, err_code: Option<&str>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.abort(err_code).await
    }

    /// Fulfills route's request with given response.
    ///
    /// An example of fulfilling all requests with 404 responses:
    ///
    /// ```js
    /// await page.route('**/*', route => {
    ///  route.fulfill({
    ///    status: 404,
    ///    contentType: 'text/plain',
    ///    body: 'Not Found!'
    ///  });
    /// });
    pub async fn fulfill_builder<'a>(
        &self,
        body: &'a str,
        is_base64: bool
    ) -> FulfillBuilder<'a, '_> {
        FulfillBuilder::new(self.inner.clone(), body, is_base64)
    }

    /// Continues route's request with optional overrides.
    ///
    /// ```js
    /// await page.route('**/*', (route, request) => {
    ///  // Override headers
    ///  const headers = {
    ///    ...request.headers(),
    ///    foo: 'bar', // set "foo" header
    ///    origin: undefined, // remove "origin" header
    ///  };
    ///  route.continue({headers});
    /// });
    /// ```
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

    /// Response headers. Header values will be converted to a string.
    pub fn headers<T>(mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>
    {
        self.args.headers = Some(x.into_iter().map(Header::from).collect());
        self
    }

    setter! {
        /// If set, equals to setting `Content-Type` response header.
        content_type: Option<&'b str>,
        /// Response status code, defaults to `200`.
        status: Option<i32>
    }

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

    /// If set changes the request HTTP headers. Header values will be converted to a string.
    pub fn headers<T>(mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>
    {
        self.args.headers = Some(x.into_iter().map(Header::from).collect());
        self
    }

    setter! {
        /// If set changes the request method (e.g. GET or POST)
        method: Option<&'b str>,
        /// If set changes the post data of request
        post_data: Option<&'c str>,
        /// If set changes the request URL. New URL must have same protocol as original one.
        url: Option<&'a str>
    }

    pub fn clear_headers(mut self) -> Self {
        self.args.headers = None;
        self
    }
}
