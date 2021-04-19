pub use crate::imp::browser_context::EventType;
use crate::{
    api::{Browser, Page},
    imp::{
        browser_context::{BrowserContext as Impl, Evt},
        core::*,
        prelude::*,
        utils::{Cookie, Geolocation, StorageState}
    },
    Error
};

/// BrowserContexts provide a way to operate multiple independent browser sessions.
///
/// If a page opens another page, e.g. with a `window.open` call, the popup will belong to the parent page's browser
/// context.
///
/// Playwright allows creation of "incognito" browser contexts with `browser.newContext()` method. "Incognito" browser
/// contexts don't write any browsing data to disk.
#[derive(Debug)]
pub struct BrowserContext {
    inner: Weak<Impl>
}

impl PartialEq for BrowserContext {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl BrowserContext {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    /// Returns all open pages in the context.
    pub fn pages(&self) -> Result<Vec<Page>, Error> {
        Ok(upgrade(&self.inner)?
            .pages()
            .iter()
            .cloned()
            .map(Page::new)
            .collect())
    }

    /// Returns the browser instance of the context. If it was launched as a persistent context None gets returned.
    pub fn browser(&self) -> Result<Option<Browser>, Error> {
        Ok(upgrade(&self.inner)?.browser().map(Browser::new))
    }

    /// Creates a new page in the browser context.
    pub async fn new_page(&self) -> Result<Page, Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        Ok(Page::new(inner.new_page().await?))
    }

    pub async fn set_default_navigation_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?
            .set_default_navigation_timeout(timeout)
            .await
    }

    pub async fn set_default_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?.set_default_timeout(timeout).await
    }

    /// If no URLs are specified, this method returns all cookies. If URLs are specified, only cookies that affect those URLs
    /// are returned.
    pub async fn cookies(&self, urls: &[String]) -> ArcResult<Vec<Cookie>> {
        upgrade(&self.inner)?.cookies(urls).await
    }

    /// Adds cookies into this browser context. All pages within this context will have these cookies installed.
    pub async fn add_cookies(&self, cookies: &[Cookie]) -> ArcResult<()> {
        upgrade(&self.inner)?.add_cookies(cookies).await
    }

    /// Clears context cookies.
    pub async fn clear_cookies(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_cookies().await
    }

    /// Grants specified permissions to the browser context. Only grants corresponding permissions to the given origin if
    /// specified.
    ///
    /// ```js
    /// const context = await browser.newContext();
    /// await context.grantPermissions(['clipboard-read']);
    /// context.clearPermissions();
    /// ```
    /// # Args
    /// ## permissions
    /// A permission or an array of permissions to grant. Permissions can be one of the following values:
    /// - `'geolocation'`
    /// - `'midi'`
    /// - `'midi-sysex'` (system-exclusive midi)
    /// - `'notifications'`
    /// - `'push'`
    /// - `'camera'`
    /// - `'microphone'`
    /// - `'background-sync'`
    /// - `'ambient-light-sensor'`
    /// - `'accelerometer'`
    /// - `'gyroscope'`
    /// - `'magnetometer'`
    /// - `'accessibility-events'`
    /// - `'clipboard-read'`
    /// - `'clipboard-write'`
    /// - `'payment-handler'`
    /// ## origin
    /// The origin to grant permissions to, e.g. `"https://example.com"`.
    pub async fn grant_permissions(
        &self,
        permissions: &[String],
        origin: Option<&str>
    ) -> ArcResult<()> {
        upgrade(&self.inner)?
            .grant_permissions(permissions, origin)
            .await
    }

    /// Clears all permission overrides for the browser context.
    pub async fn clear_permissions(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_permissions().await
    }

    /// Sets the context's geolocation. Passing `null` or `undefined` emulates position unavailable.
    ///
    /// ```js
    /// await browserContext.setGeolocation({latitude: 59.95, longitude: 30.31667});
    /// ```
    /// > NOTE: Consider using [`method: BrowserContext.grantPermissions`] to grant permissions for the browser context pages to
    /// read its geolocation.
    pub async fn set_geolocation(&self, geolocation: Option<&Geolocation>) -> ArcResult<()> {
        upgrade(&self.inner)?.set_geolocation(geolocation).await
    }

    /// Sets whether to emulate network being offline for the browser context.
    pub async fn set_offline(&self, offline: bool) -> ArcResult<()> {
        upgrade(&self.inner)?.set_offline(offline).await
    }

    /// Adds a script which would be evaluated in one of the following scenarios:
    /// - Whenever a page is created in the browser context or is navigated.
    /// - Whenever a child frame is attached or navigated in any page in the browser context. In this case, the script is
    ///  evaluated in the context of the newly attached frame.
    ///
    /// The script is evaluated after the document was created but before any of its scripts were run. This is useful to amend
    /// the JavaScript environment, e.g. to seed `Math.random`.
    ///
    /// An example of overriding `Math.random` before the page loads:
    ///
    /// ```js browser
    ///// preload.js
    /// Math.random = () => 42;
    /// ```
    /// ```js
    ///// In your playwright script, assuming the preload.js file is in same directory.
    /// await browserContext.addInitScript({
    ///  path: 'preload.js'
    /// });
    /// ```
    /// > NOTE: The order of evaluation of multiple scripts installed via [`method: BrowserContext.addInitScript`] and
    /// [`method: Page.addInitScript`] is not defined.
    pub async fn add_init_script(&self, script: &str) -> ArcResult<()> {
        // arg not supported
        upgrade(&self.inner)?.add_init_script(script).await
    }

    /// The extra HTTP headers will be sent with every request initiated by any page in the context. These headers are merged
    /// with page-specific extra HTTP headers set with [`method: Page.setExtraHTTPHeaders`]. If page overrides a particular
    /// header, page-specific header value will be used instead of the browser context header value.
    ///
    /// > NOTE: [`method: BrowserContext.setExtraHTTPHeaders`] does not guarantee the order of headers in the outgoing requests.
    pub async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        upgrade(&self.inner)?.set_extra_http_headers(headers).await
    }

    // async fn expose_binding(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn expose_function(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn route(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn unroute(&mut self) -> Result<(), Error> { unimplemented!() }

    pub async fn expect_event(&self, evt: EventType) -> Result<Event, Error> {
        upgrade(&self.inner)?
            .expect_event(evt)
            .await
            .map(Event::from)
    }

    /// Returns storage state for this browser context, contains current cookies and local storage snapshot.
    pub async fn storage_state(&self) -> ArcResult<StorageState> {
        // path no supported
        upgrade(&self.inner)?.storage_state().await
    }

    /// All temporary browsers will be closed when the connection is terminated, but
    /// this struct has no Drop. it needs to be called explicitly to close it at any given time.
    /// > NOTE: The default browser context cannot be closed.
    pub async fn close(&self) -> ArcResult<()> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close().await
    }

    pub async fn pause(&self) -> ArcResult<()> { upgrade(&self.inner)?.pause().await }

    subscribe_event! {}

    // background_page for chromium
    // new_cdp_session
    // service_workers
}

#[derive(Debug, PartialEq)]
pub enum Event {
    // BackgroundPage for chromium persistent
    // ServiceWorker
    /// Emitted when Browser context gets closed. This might happen because of one of the following:
    /// - Browser context is closed.
    /// - Browser application is closed or crashed.
    /// - The [`method: Browser.close`] method was called.
    Close,
    /// The event is emitted when a new Page is created in the BrowserContext. The page may still be loading. The event will
    /// also fire for popup pages. See also [`event: Page.popup`] to receive events about popups relevant to a specific page.
    ///
    /// The earliest moment that page is available is when it has navigated to the initial url. For example, when opening a
    /// popup with `window.open('http://example.com')`, this event will fire when the network request to <http://example.com> is
    /// done and its response has started loading in the popup.
    ///
    /// ```js
    /// const [newPage] = await Promise.all([
    ///  context.waitForEvent('page'),
    ///  page.click('a[target=_blank]'),
    /// ]);
    /// console.log(await newPage.evaluate('location.href'));
    /// ```
    Page(Page)
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Event {
        match e {
            Evt::Close => Event::Close,
            Evt::Page(w) => Event::Page(Page::new(w))
        }
    }
}
