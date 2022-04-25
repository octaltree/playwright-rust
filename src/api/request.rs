use crate::{
    api::{Frame, Response},
    imp::{core::*, prelude::*, request::Request as Impl, utils::ResponseTiming}
};

/// Whenever the page sends a request for a network resource the following sequence of events are emitted by `Page`:
/// - [`event: Page.request`] emitted when the request is issued by the page.
/// - [`event: Page.response`] emitted when/if the response status and headers are received for the request.
/// - [`event: Page.requestFinished`] emitted when the response body is downloaded and the request is complete.
///
/// If request fails at some point, then instead of `'requestfinished'` event (and possibly instead of 'response' event),
/// the  [`event: Page.requestFailed`] event is emitted.
///
/// > NOTE: HTTP Error responses, such as 404 or 503, are still successful responses from HTTP standpoint, so request will
/// complete with `'requestfinished'` event.
///
/// If request gets a 'redirect' response, the request is successfully finished with the 'requestfinished' event, and a new
/// request is  issued to a redirected url.
#[derive(Debug, Clone)]
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

    /// Request's method (GET, POST, etc.)
    pub fn method(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.method().into()) }

    /// Contains the request's resource type as it was perceived by the rendering engine. ResourceType will be one of the
    /// following: `document`, `stylesheet`, `image`, `media`, `font`, `script`, `texttrack`, `xhr`, `fetch`, `eventsource`,
    /// `websocket`, `manifest`, `other`.
    pub fn resource_type(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.resource_type().into())
    }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().into()) }

    /// Whether this request is driving frame's navigation.
    pub fn is_navigation_request(&self) -> Result<bool, Error> {
        Ok(upgrade(&self.inner)?.is_navigation_request())
    }

    /// Returns the `Frame` that initiated this request.
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

    /// An object with HTTP headers associated with the request. All header names are lower-case.
    pub fn headers(&self) -> Result<HashMap<String, String>, Error> {
        Ok(upgrade(&self.inner)?.headers().clone())
    }

    /// Request that was redirected by the server to this one, if any.
    ///
    /// When the server responds with a redirect, Playwright creates a new `Request` object. The two requests are connected by
    /// `redirectedFrom()` and `redirectedTo()` methods. When multiple server redirects has happened, it is possible to
    /// construct the whole redirect chain by repeatedly calling `redirectedFrom()`.
    ///
    /// For example, if the website `http://example.com` redirects to `https://example.com`:
    ///
    /// ```js
    /// const response = await page.goto('http://example.com');
    /// console.log(response.request().redirectedFrom().url()); // 'http://example.com'
    /// ```
    ///
    /// If the website `https://google.com` has no redirects:
    ///
    /// ```js
    /// const response = await page.goto('https://google.com');
    /// console.log(response.request().redirectedFrom()); // null
    /// ```
    pub fn redirected_from(&self) -> Result<Option<Request>, Error> {
        Ok(upgrade(&self.inner)?.redirected_from().map(Request::new))
    }

    pub async fn redirected_to(&self) -> Result<Option<Request>, Error> {
        Ok(upgrade(&self.inner)?.redirected_to().map(Request::new))
    }

    /// Returns the matching `Response` object, or `null` if the response was not received due to error.
    pub async fn response(&self) -> Result<Option<Response>, Arc<Error>> {
        Ok(upgrade(&self.inner)?.response().await?.map(Response::new))
    }

    /// The method returns `null` unless this request has failed, as reported by `requestfailed` event.
    ///
    /// Example of logging of all the failed requests:
    ///
    /// ```js
    /// page.on('requestfailed', request => {
    ///  console.log(request.url() + ' ' + request.failure().errorText);
    /// });
    /// ```
    pub fn failure(&self) -> Result<Option<String>, Error> { Ok(upgrade(&self.inner)?.failure()) }

    /// Returns resource timing information for given request. Most of the timing values become available upon the response,
    /// `responseEnd` becomes available when request finishes. Find more information at
    /// [Resource Timing API](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming).
    ///
    /// ```js
    /// const [request] = await Promise.all([
    ///  page.waitForEvent('requestfinished'),
    ///  page.goto('http://example.com')
    /// ]);
    /// console.log(request.timing());
    /// ```
    pub fn timing(&self) -> Result<Option<ResponseTiming>, Error> {
        Ok(upgrade(&self.inner)?.timing())
    }

    pub fn response_end(&self) -> Result<Option<f64>, Error> {
        Ok(upgrade(&self.inner)?.response_end())
    }
}
