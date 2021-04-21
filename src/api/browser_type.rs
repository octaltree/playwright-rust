pub use crate::imp::browser_type::{RecordHar, RecordVideo};
use crate::{
    api::{browser::Browser, browser_context::BrowserContext, playwright::DeviceDescriptor},
    imp::{
        browser_type::{BrowserType as Impl, LaunchArgs, LaunchPersistentContextArgs},
        core::*,
        prelude::*,
        utils::{
            BrowserChannel, ColorScheme, Geolocation, HttpCredentials, ProxySettings, Viewport
        }
    },
    Error
};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Weak<Impl>
}

impl BrowserType {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    /// Returns browser name. For example: `'chromium'`, `'webkit'` or `'firefox'`.
    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn name(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.name().into()) }

    /// A path where Playwright expects to find a bundled browser executable.
    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn executable(&self) -> Result<PathBuf, Error> {
        Ok(upgrade(&self.inner)?.executable().into())
    }

    /// launch [`Browser`]
    /// Returns the browser instance.
    ///
    /// You can use `ignoreDefaultArgs` to filter out `--mute-audio` from default arguments:
    ///
    /// ```js
    /// const browser = await chromium.launch({  // Or 'firefox' or 'webkit'.
    ///  ignoreDefaultArgs: ['--mute-audio']
    /// });
    /// ```
    ///
    /// > **Chromium-only** Playwright can also be used to control the Google Chrome or Microsoft Edge browsers, but it works
    /// best with the version of Chromium it is bundled with. There is no guarantee it will work with any other version. Use
    /// `executablePath` option with extreme caution.
    /// >
    /// > If Google Chrome (rather than Chromium) is preferred, a
    /// [Chrome Canary](https://www.google.com/chrome/browser/canary.html) or
    /// [Dev Channel](https://www.chromium.org/getting-involved/dev-channel) build is suggested.
    /// >
    /// > Stock browsers like Google Chrome and Microsoft Edge are suitable for tests that require proprietary media codecs for
    /// video playback. See
    /// [this article](https://www.howtogeek.com/202825/what%E2%80%99s-the-difference-between-chromium-and-chrome/) for other
    /// differences between Chromium and Chrome.
    /// [This article](https://chromium.googlesource.com/chromium/src/+/lkgr/docs/chromium_browser_vs_google_chrome.md)
    /// describes some differences for Linux users.
    pub fn launcher(&self) -> Launcher<'_, '_, '_> { Launcher::new(self.inner.clone()) }

    /// launch_persistent_context [`BrowserContext`]
    /// Returns the persistent browser context instance.
    ///
    /// Launches browser that uses persistent storage located at `userDataDir` and returns the only context. Closing this
    /// context will automatically close the browser.
    /// user_data_dir: Path to a User Data Directory, which stores browser session data like cookies and local storage. More details for
    /// [Chromium](https://chromium.googlesource.com/chromium/src/+/master/docs/user_data_dir.md#introduction) and
    /// [Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Command_Line_Options#User_Profile). Note that Chromium's user
    /// data directory is the **parent** directory of the "Profile Path" seen at `chrome://version`.
    pub fn persistent_context_launcher<'a>(
        &self,
        user_data_dir: &'a Path
    ) -> PersistentContextLauncher<'a, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        PersistentContextLauncher::new(self.inner.clone(), user_data_dir)
    }

    // connect
    // connect_over_cdp
    // launch_server
}

/// [`BrowserType::launcher`]
pub struct Launcher<'a, 'b, 'c> {
    inner: Weak<Impl>,
    args: LaunchArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> Launcher<'a, 'b, 'c> {
    pub async fn launch(self) -> Result<Browser, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch(args).await?;
        Ok(Browser::new(r))
    }

    fn new(inner: Weak<Impl>) -> Self {
        Launcher {
            inner,
            args: LaunchArgs::default()
        }
    }

    setter! {
        /// Path to a browser executable to run instead of the bundled one. If `executablePath` is a relative path, then it is
        /// resolved relative to the current working directory. Note that Playwright only works with the bundled Chromium, Firefox
        /// or WebKit, use at your own risk.
        executable: Option<&'a Path>,
        /// Additional arguments to pass to the browser instance. The list of Chromium flags can be found
        /// [here](http://peter.sh/experiments/chromium-command-line-switches/).
        args: Option<&'b [String]>,
        /// If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;
        /// use with care. Defaults to `false`.
        ignore_all_default_args: Option<bool>,
        /// Close the browser process on Ctrl-C. Defaults to `true`.
        handle_sigint: Option<bool>,
        /// Close the browser process on SIGTERM. Defaults to `true`.
        handle_sigterm: Option<bool>,
        /// Close the browser process on SIGHUP. Defaults to `true`.
        handle_sighup: Option<bool>,
        /// Maximum time in milliseconds to wait for the browser instance to start. Defaults to `30000` (30 seconds). Pass `0` to
        /// disable timeout.
        timeout: Option<f64>,
        /// **Chromium-only** Whether to auto-open a Developer Tools panel for each tab. If this option is `true`, the `headless`
        /// option will be set `false`.
        devtools: Option<bool>,
        /// Network proxy settings.
        proxy: Option<ProxySettings>,
        /// If specified, accepted downloads are downloaded into this directory. Otherwise, temporary directory is created and is
        /// deleted when browser is closed.
        downloads: Option<&'c Path>,
        /// Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.
        slowmo: Option<f64>,
        /// Specify environment variables that will be visible to the browser. Defaults to `process.env`.
        env: Option<Map<String, Value>>,
        /// Whether to run browser in headless mode. More details for
        /// [Chromium](https://developers.google.com/web/updates/2017/04/headless-chrome) and
        /// [Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Headless_mode). Defaults to `true` unless the
        /// `devtools` option is `true`.
        headless: Option<bool>,
        /// Enable Chromium sandboxing. Defaults to `false`.
        chromium_sandbox: Option<bool>,
        /// Firefox user preferences. Learn more about the Firefox user preferences at
        /// [`about:config`](https://support.mozilla.org/en-US/kb/about-config-editor-firefox).
        firefox_user_prefs: Option<Map<String, Value>>,
        channel: Option<BrowserChannel>
    }
    //#[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. If an array is\ngiven, then filters out the given default arguments. Dangerous option; use with care. Defaults to `false`."]
    // ignore_default_args: Option<NotImplementedYet>,
    //#[doc = "Logger sink for Playwright logging."]
    // logger: Option<Logger>,
}

/// [`BrowserType::persistent_context_launcher`]
///
/// Has launch args and context args
pub struct PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    inner: Weak<Impl>,
    args: LaunchPersistentContextArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    pub async fn launch(self) -> Result<BrowserContext, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch_persistent_context(args).await?;
        Ok(BrowserContext::new(r))
    }

    fn new(inner: Weak<Impl>, user_data_dir: &'a Path) -> Self {
        Self {
            inner,
            args: LaunchPersistentContextArgs::new(user_data_dir)
        }
    }

    pub fn set_device(self, device: &'e DeviceDescriptor) -> Self {
        DeviceDescriptor::set_persistent_context(&device, self)
    }

    setter! {
        /// Path to a browser executable to run instead of the bundled one. If `executablePath` is a relative path, then it is
        /// resolved relative to the current working directory. **BEWARE**: Playwright is only guaranteed to work with the bundled
        /// Chromium, Firefox or WebKit, use at your own risk.
        executable: Option<&'b Path>,
        /// Additional arguments to pass to the browser instance. The list of Chromium flags can be found
        /// [here](http://peter.sh/experiments/chromium-command-line-switches/).
        args: Option<&'c [String]>,
        /// If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;
        /// use with care. Defaults to `false`.
        ignore_all_default_args: Option<bool>,
        /// Close the browser process on SIGHUP. Defaults to `true`.
        handle_sighup: Option<bool>,
        /// Close the browser process on Ctrl-C. Defaults to `true`.
        handle_sigint: Option<bool>,
        /// Close the browser process on SIGTERM. Defaults to `true`.
        handle_sigterm: Option<bool>,
        /// Maximum time in milliseconds to wait for the browser instance to start. Defaults to `30000` (30 seconds). Pass `0` to
        /// disable timeout.
        timeout: Option<f64>,
        /// Specify environment variables that will be visible to the browser. Defaults to `process.env`.
        env: Option<Map<String, Value>>,
        /// Whether to run browser in headless mode. More details for
        /// [Chromium](https://developers.google.com/web/updates/2017/04/headless-chrome) and
        /// [Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Firefox/Headless_mode). Defaults to `true` unless the
        /// `devtools` option is `true`.
        headless: Option<bool>,
        /// **Chromium-only** Whether to auto-open a Developer Tools panel for each tab. If this option is `true`, the `headless`
        /// option will be set `false`.
        devtools: Option<bool>,
        /// Network proxy settings.
        proxy: Option<ProxySettings>,
        /// If specified, accepted downloads are downloaded into this directory. Otherwise, temporary directory is created and is
        /// deleted when browser is closed.
        downloads: Option<&'d Path>,
        /// Slows down Playwright operations by the specified amount of milliseconds. Useful so that you can see what is going on.
        /// Defaults to 0.
        slowmo: Option<f64>,
        /// Emulates consistent viewport for each page. Defaults to an 1280x720 viewport. `null` disables the default viewport.
        viewport: Option<Option<Viewport>>,
        /// Does not enforce fixed viewport, allows resizing window in the headed mode.
        no_viewport: Option<bool>,
        /// Emulates consistent window screen size available inside web page via `window.screen`. Is only used when the `viewport`
        /// is set.
        screen: Option<Viewport>,
        /// Whether to ignore HTTPS errors during navigation. Defaults to `false`.
        ignore_https_errors: Option<bool>,
        /// Whether or not to enable JavaScript in the context. Defaults to `true`.
        js_enabled: Option<bool>,
        /// Toggles bypassing page's Content-Security-Policy.
        bypass_csp: Option<bool>,
        /// Specific user agent to use in this context.
        user_agent: Option<&'e str>,
        /// Specify user locale, for example `en-GB`, `de-DE`, etc. Locale will affect `navigator.language` value, `Accept-Language`
        /// request header value as well as number and date formatting rules.
        locale: Option<&'f str>,
        /// Changes the timezone of the context. See
        /// [ICU's metaZones.txt](https://cs.chromium.org/chromium/src/third_party/icu/source/data/misc/metaZones.txt?rcl=faee8bc70570192d82d2978a71e2a615788597d1)
        /// for a list of supported timezone IDs.
        timezone_id: Option<&'g str>,
        geolocation: Option<Geolocation>,
        /// A list of permissions to grant to all pages in this context. See [`method: BrowserContext.grantPermissions`] for more
        /// details.
        permissions: Option<&'h [String]>,
        /// An object containing additional HTTP headers to be sent with every request. All header values must be strings.
        extra_http_headers: Option<HashMap<String, String>>,
        /// Whether to emulate network being offline. Defaults to `false`.
        offline: Option<bool>,
        /// Credentials for [HTTP authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication).
        http_credentials: Option<&'i HttpCredentials>,
        /// Specify device scale factor (can be thought of as dpr). Defaults to `1`.
        device_scale_factor: Option<f64>,
        /// Whether the `meta viewport` tag is taken into account and touch events are enabled. Defaults to `false`. Not supported
        /// in Firefox.
        is_mobile: Option<bool>,
        /// Specifies if viewport supports touch events. Defaults to false.
        has_touch: Option<bool>,
        /// Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`. See
        /// [`method: Page.emulateMedia`] for more details. Defaults to `'light'`.
        color_scheme: Option<ColorScheme>,
        /// Whether to automatically download all the attachments. Defaults to `false` where all the downloads are canceled.
        accept_downloads: Option<bool>,
        /// Enable Chromium sandboxing. Defaults to `true`.
        chromium_sandbox: Option<bool>,
        /// Enables video recording for all pages into `recordVideo.dir` directory. If not specified videos are not recorded. Make
        /// sure to await [`method: BrowserContext.close`] for videos to be saved.
        record_video: Option<RecordVideo<'j>>,
        /// Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into `recordHar.path` file. If not
        /// specified, the HAR is not recorded. Make sure to await [`method: BrowserContext.close`] for the HAR to be saved.
        record_har: Option<RecordHar<'k>>,
        channel: Option<BrowserChannel>
    }
    //#[doc = "If `true`, Playwright does not pass its own configurations args and only uses the ones from `args`. Dangerous option;\nuse with care."]
    // ignore_default_args: Option<Vec<String>>,
    //#[doc = "Logger sink for Playwright logging."] logger: Option<Logger>,
    //#[doc = "Optional setting to control whether to omit request content from the HAR. Defaults to `false`."]
    // record_har_omit_content: Option<bool>,
    //#[doc = "Enables [HAR](http://www.softwareishard.com/blog/har-12-spec) recording for all pages into the specified HAR file on the\nfilesystem. If not specified, the HAR is not recorded. Make sure to call [`method: BrowserContext.close`] for the HAR to\nbe saved."]
    // record_har_path: Option<path>,
    //#[doc = "Enables video recording for all pages into the specified directory. If not specified videos are not recorded. Make sure\nto call [`method: BrowserContext.close`] for videos to be saved."]
    // record_video_dir: Option<path>,
    //#[doc = "Dimensions of the recorded videos. If not specified the size will be equal to `viewport` scaled down to fit into\n800x800. If `viewport` is not configured explicitly the video size defaults to 800x450. Actual picture of each page will\nbe scaled down if necessary to fit the specified size."]
    // record_video_size: Option<NotImplementedYet>,
    //#[doc = "**DEPRECATED** Use `recordVideo` instead."] video_size: Option<NotImplementedYet>,
    //#[doc = "**DEPRECATED** Use `recordVideo` instead."] videos_path: Option<path>,
}
