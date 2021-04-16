pub use crate::imp::browser_type::{RecordHar, RecordVideo};
use crate::{
    api::BrowserContext,
    imp::{
        self,
        browser::NewContextArgs,
        core::*,
        prelude::*,
        utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, StorageState, Viewport}
    },
    Error
};

pub struct Browser {
    inner: Weak<imp::browser::Browser>
}

impl PartialEq for Browser {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl Browser {
    pub(crate) fn new(inner: Weak<imp::browser::Browser>) -> Self { Self { inner } }

    /// Returns an array of all open browser contexts. In a newly created browser, this will return zero browser contexts.
    ///
    /// ```js
    /// const browser = await pw.webkit.launch();
    /// console.log(browser.contexts().length); // prints `0`
    ///
    /// const context = await browser.newContext();
    /// console.log(browser.contexts().length); // prints `1`
    /// ```
    pub fn contexts(&self) -> Result<Vec<BrowserContext>, Error> {
        Ok(upgrade(&self.inner)?
            .contexts()
            .iter()
            .cloned()
            .map(BrowserContext::new)
            .collect())
    }

    /// Returns the browser version.
    pub fn version(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.version().to_owned())
    }

    pub fn exists(&self) -> bool { self.inner.upgrade().is_some() }

    /// new_context [`BrowserContext`]
    /// Creates a new browser context. It won't share cookies/cache with other browser contexts.
    pub fn context_builder(&self) -> ContextBuilder<'_, '_, '_, '_, '_, '_, '_> {
        ContextBuilder::new(self.inner.clone())
    }

    /// All temporary browsers will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
    pub async fn close(&self) -> Result<(), Arc<Error>> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close().await
    }
}

// TODO: async drop

/// [`Browser::context_builder`]
pub struct ContextBuilder<'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    inner: Weak<imp::browser::Browser>,
    args: NewContextArgs<'e, 'f, 'g, 'h, 'i, 'j, 'k>
}

impl<'e, 'f, 'g, 'h, 'i, 'j, 'k> ContextBuilder<'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    pub async fn build(self) -> Result<BrowserContext, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.new_context(args).await?;
        Ok(BrowserContext::new(r))
    }

    fn new(inner: Weak<imp::browser::Browser>) -> Self {
        Self {
            inner,
            args: NewContextArgs::default()
        }
    }

    setter! {
        /// Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled.
        accept_downloads: Option<bool>,
        /// Toggles bypassing page's Content-Security-Policy.
        bypass_csp: Option<bool>,
        /// Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See
        /// [`method: Page.emulateMedia`] for more details. Defaults to `'light'`.
        color_scheme: Option<ColorScheme>,
        /// Specify device scale factor (can be thought of as dpr). Defaults to `1`.
        device_scale_factor: Option<f64>,
        /// An object containing additional HTTP headers to be sent with every request. All header values must be strings.
        extra_http_headers: Option<HashMap<String, String>>,
        geolocation: Option<Geolocation>,
        has_touch: Option<bool>,
        /// Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication).
        http_credentials: Option<&'i HttpCredentials>,
        /// Whether to ignore HTTPS errors during navigation. Defaults to `false`.
        ignore_https_errors: Option<bool>,
        /// Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported
        /// in Firefox.
        is_mobile: Option<bool>,
        /// Whether or not to enable JavaScript in the context. Defaults to `true`.
        js_enabled: Option<bool>,
        /// Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`
        /// request header value as well as number and date formatting rules.
        locale: Option<&'f str>,
        /// Does not enforce fixed viewport, allows resizing window in the headed mode.
        no_viewport: Option<bool>,
        /// Whether to emulate network being offline. Defaults to `false`.
        offline: Option<bool>,
        /// A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more
        /// details.
        permissions: Option<&'h [String]>,
        /// Network proxy settings to use with this context. Note that browser needs to be launched with the global proxy for this
        /// option to work. If all contexts override the proxy, global proxy will be never used and can be any string, for example
        /// `launch({ proxy: { server: 'per-context' } })`.
        proxy: Option<ProxySettings>,
        /// Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not
        /// specified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved.
        record_har: Option<RecordHar<'k>>,
        /// Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make
        /// sure to await [`method: BrowserContext.close`] for videos to be saved.
        record_video: Option<RecordVideo<'j>>,
        /// Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`
        /// is set.
        screen: Option<Viewport>,
        /// Populates context with given storage state. This option can be used to initialize context with logged-in information
        /// obtained via [`method: BrowserContext.storageState`]. Either a path to the file with saved storage, or an object with
        /// the following fields:
        storage_state: Option<StorageState>,
        /// Changes the timezone of the context. See
        /// [ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)
        /// for a list of supported timezone IDs.
        timezone_id: Option<&'g str>,
        /// Specific user agent to use in this context.
        user_agent: Option<&'e str>,
        /// Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport.
        viewport: Option<Option<Viewport>>
    }
    ///// Logger sink for Playwright logging.
    // logger: Option<Logger>,
}
