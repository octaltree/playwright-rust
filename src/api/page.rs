pub use crate::{
    api::{
        frame::{
            AddScriptTagBuilder, CheckBuilder, ClickBuilder, DblClickBuilder, FillBuilder,
            GotoBuilder, HoverBuilder, PressBuilder, SelectOptionBuilder, SetContentBuilder,
            SetInputFilesBuilder, TapBuilder, TypeBuilder, UncheckBuilder, WaitForFunctionBuilder,
            WaitForSelectorBuilder
        },
        Download, JsHandle, Request
    },
    imp::page::{EventType, Media}
};
use crate::{
    api::{
        input_device::*, Accessibility, BrowserContext, ConsoleMessage, ElementHandle, Frame,
        Keyboard, Response, TouchScreen, Video, WebSocket, Worker
    },
    imp::{
        core::*,
        frame::Frame as FrameImpl,
        page::{EmulateMediaArgs, Evt, Page as Impl, PdfArgs, ReloadArgs, ScreenshotArgs},
        prelude::*,
        utils::{ColorScheme, File, FloatRect, Length, PdfMargins, ScreenshotType, Viewport}
    },
    protocol::generated::LifecycleEvent,
    Error
};
use std::fmt::{Debug, Formatter};

/// Page provides methods to interact with a single tab in a `Browser`, or an
/// [extension background page](https://developer.chrome.com/extensions/background_pages) in Chromium. One `Browser`
/// instance might have multiple `Page` instances.
///
/// This example creates a page, navigates it to a URL, and then saves a screenshot:
///
/// ```js
/// const { webkit } = require('playwright');  // Or 'chromium' or 'firefox'.
///
/// (async () => {
///  const browser = await webkit.launch();
///  const context = await browser.newContext();
///  const page = await context.newPage();
///  await page.goto('https://example.com');
///  await page.screenshot({path: 'screenshot.png'});
///  await browser.close();
/// })();
/// ```
///
/// The Page class emits various events (described below) which can be handled using any of Node's native
/// [`EventEmitter`](https://nodejs.org/api/events.html#events_class_eventemitter) methods, such as `on`, `once` or
/// `removeListener`.
///
/// This example logs a message for a single page `load` event:
///
/// ```js
/// page.once('load', () => console.log('Page loaded!'));
/// ```
///
/// To unsubscribe from events use the `removeListener` method:
///
/// ```js
/// function logRequest(interceptedRequest) {
///  console.log('A request was made:', interceptedRequest.url());
/// }
/// page.on('request', logRequest);
///// Sometime later...
/// page.removeListener('request', logRequest);
/// ```
#[derive(Debug, Clone)]
pub struct Page {
    inner: Weak<Impl>,
    pub keyboard: Keyboard,
    pub touch_screen: TouchScreen,
    pub mouse: Mouse,
    pub accessibility: Accessibility
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl Page {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        Self {
            inner: inner.clone(),
            keyboard: Keyboard::new(inner.clone()),
            touch_screen: TouchScreen::new(inner.clone()),
            mouse: Mouse::new(inner.clone()),
            accessibility: Accessibility::new(inner)
        }
    }

    pub fn context(&self) -> BrowserContext {
        BrowserContext::new(weak_and_then(&self.inner, |rc| rc.browser_context()))
    }

    fn main_frame_weak(&self) -> Weak<FrameImpl> {
        weak_and_then(&self.inner, |rc| rc.main_frame())
    }

    /// The page's main frame. Page is guaranteed to have a main frame which persists during navigations.
    pub fn main_frame(&self) -> Frame { Frame::new(self.main_frame_weak()) }

    /// An array of all frames attached to the page.
    pub fn frames(&self) -> Result<Vec<Frame>, Error> {
        Ok(upgrade(&self.inner)?
            .frames()
            .into_iter()
            .map(Frame::new)
            .collect())
    }

    /// This method returns all of the dedicated [WebWorkers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API)
    /// associated with the page.
    ///
    /// > NOTE: This does not contain ServiceWorkers
    pub fn workers(&self) -> Result<Vec<Worker>, Error> {
        Ok(upgrade(&self.inner)?
            .workers()
            .into_iter()
            .map(Worker::new)
            .collect())
    }

    /// Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the
    /// last redirect.
    pub fn reload_builder(&self) -> ReloadBuilder { ReloadBuilder::new(self.inner.clone()) }
    /// Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the
    /// last redirect. If can not go back, returns `null`.
    ///
    /// Navigate to the previous page in history.
    pub fn go_back_builder(&self) -> GoBackBuilder { GoBackBuilder::new(self.inner.clone()) }
    /// Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the
    /// last redirect. If can not go forward, returns `null`.
    ///
    /// Navigate to the next page in history.
    pub fn go_forward_builder(&self) -> GoForwardBuilder {
        GoForwardBuilder::new(self.inner.clone())
    }

    pub async fn set_default_navigation_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?
            .set_default_navigation_timeout(timeout)
            .await
    }

    pub async fn set_default_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?.set_default_timeout(timeout).await
    }

    pub fn viewport_size(&self) -> Result<Option<Viewport>, Error> {
        Ok(upgrade(&self.inner)?.viewport_size())
    }

    /// In the case of multiple pages in a single browser, each page can have its own viewport size. However,
    /// [`method: Browser.newContext`] allows to set viewport size (and more) for all pages in the context at once.
    ///
    /// `page.setViewportSize` will resize the page. A lot of websites don't expect phones to change size, so you should set the
    /// viewport size before navigating to the page.
    pub async fn set_viewport_size(&self, viewport_size: Viewport) -> ArcResult<()> {
        upgrade(&self.inner)?.set_viewport_size(viewport_size).await
    }

    /// Video object associated with this page.
    pub fn video(&self) -> Result<Option<Video>, Error> {
        Ok(upgrade(&self.inner)?.video().map(Video::new))
    }

    ///// Returns frame matching the specified criteria. Either `name` or `url` must be specified.
    // fn frame(&self) -> Option<Frame> { unimplemented!() }

    /// Brings page to front (activates tab).
    pub async fn bring_to_front(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.bring_to_front().await
    }

    /// Adds a script which would be evaluated in one of the following scenarios:
    /// - Whenever the page is navigated.
    /// - Whenever the child frame is attached or navigated. In this case, the script is evaluated in the context of the newly
    ///  attached frame.
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
    ///// In your playwright script, assuming the preload.js file is in same directory
    /// await page.addInitScript({ path: './preload.js' });
    /// ```
    /// 
    /// > NOTE: The order of evaluation of multiple scripts installed via [`method: BrowserContext.addInitScript`] and
    /// [`method: Page.addInitScript`] is not defined.
    pub async fn add_init_script(&self, source: &str) -> ArcResult<()> {
        // arg not supported
        upgrade(&self.inner)?.add_init_script(source).await
    }

    /// Returns the PDF buffer.
    ///
    /// > NOTE: Generating a pdf is currently only supported in Chromium headless.
    ///
    /// `page.pdf()` generates a pdf of the page with `print` css media. To generate a pdf with `screen` media, call
    /// [`method: Page.emulateMedia`] before calling `page.pdf()`:
    ///
    /// > NOTE: By default, `page.pdf()` generates a pdf with modified colors for printing. Use the
    /// [`-webkit-print-color-adjust`](https://developer.mozilla.org/en-US/docs/Web/CSS/-webkit-print-color-adjust) property to
    /// force rendering of exact colors.
    ///
    /// ```js
    ///// Generates a PDF with 'screen' media type.
    /// await page.emulateMedia({media: 'screen'});
    /// await page.pdf({path: 'page.pdf'});
    /// ```
    /// 
    /// The `width`, `height`, and `margin` options accept values labeled with units. Unlabeled values are treated as pixels.
    ///
    /// A few examples:
    /// - `page.pdf({width: 100})` - prints with width set to 100 pixels
    /// - `page.pdf({width: '100px'})` - prints with width set to 100 pixels
    /// - `page.pdf({width: '10cm'})` - prints with width set to 10 centimeters.
    ///
    /// All possible units are:
    /// - `px` - pixel
    /// - `in` - inch
    /// - `cm` - centimeter
    /// - `mm` - millimeter
    ///
    /// The `format` options are:
    /// - `Letter`: 8.5in x 11in
    /// - `Legal`: 8.5in x 14in
    /// - `Tabloid`: 11in x 17in
    /// - `Ledger`: 17in x 11in
    /// - `A0`: 33.1in x 46.8in
    /// - `A1`: 23.4in x 33.1in
    /// - `A2`: 16.54in x 23.4in
    /// - `A3`: 11.7in x 16.54in
    /// - `A4`: 8.27in x 11.7in
    /// - `A5`: 5.83in x 8.27in
    /// - `A6`: 4.13in x 5.83in
    ///
    /// > NOTE: `headerTemplate` and `footerTemplate` markup have the following limitations: > 1. Script tags inside templates
    /// are not evaluated. > 2. Page styles are not visible inside templates.
    pub fn pdf_builder(&self) -> PdfBuilder<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        PdfBuilder::new(self.inner.clone())
    }

    /// All temporary pages will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
    /// If `runBeforeUnload` is `false`, does not run any unload handlers and waits for the page to be closed. If
    /// `runBeforeUnload` is `true` the method will run unload handlers, but will **not** wait for the page to close.
    ///
    /// By default, `page.close()` **does not** run `beforeunload` handlers.
    ///
    /// > NOTE: if `runBeforeUnload` is passed as true, a `beforeunload` dialog might be summoned and should be handled manually
    /// via [`event: Page.dialog`] event.
    pub async fn close(&self, run_before_unload: Option<bool>) -> ArcResult<()> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close(run_before_unload).await
    }

    pub fn screenshot_builder(&self) -> ScreenshotBuilder {
        ScreenshotBuilder::new(self.inner.clone())
    }

    /// This method changes the `CSS media type` through the `media` argument, and/or the `'prefers-colors-scheme'` media
    /// feature, using the `colorScheme` argument.
    ///
    /// ```js
    /// await page.evaluate(() => matchMedia('screen').matches);
    ///// → true
    /// await page.evaluate(() => matchMedia('print').matches);
    ///// → false
    /// await page.emulateMedia({ media: 'print' });
    /// await page.evaluate(() => matchMedia('screen').matches);
    ///// → false
    /// await page.evaluate(() => matchMedia('print').matches);
    ///// → true
    /// await page.emulateMedia({});
    /// await page.evaluate(() => matchMedia('screen').matches);
    ///// → true
    /// await page.evaluate(() => matchMedia('print').matches);
    ///// → false
    /// ```
    /// ```js
    /// await page.emulateMedia({ colorScheme: 'dark' });
    /// await page.evaluate(() => matchMedia('(prefers-color-scheme: dark)').matches);
    ///// → true
    /// await page.evaluate(() => matchMedia('(prefers-color-scheme: light)').matches);
    ///// → false
    /// await page.evaluate(() => matchMedia('(prefers-color-scheme: no-preference)').matches);
    ///// → false
    /// ```
    pub fn emulate_media_builder(&self) -> EmulateMediaBuilder {
        EmulateMediaBuilder::new(self.inner.clone())
    }

    /// Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`.
    pub async fn opener(&self) -> ArcResult<Option<Page>> {
        Ok(upgrade(&self.inner)?.opener().await?.map(Page::new))
    }

    /// The extra HTTP headers will be sent with every request the page initiates.
    ///
    /// > NOTE: [`method: Page.setExtraHTTPHeaders`] does not guarantee the order of headers in the outgoing requests.
    pub async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        upgrade(&self.inner)?.set_extra_http_headers(headers).await
    }

    pub async fn expect_event(&self, evt: EventType) -> Result<Event, Error> {
        let stream = upgrade(&self.inner)?.subscribe_event();
        let timeout = upgrade(&self.inner)?.default_timeout();
        expect_event(stream, evt, timeout).await.map(Event::from)
    }

    subscribe_event! {}

    // coverage
    // expose_binding
    // expose_function
    // route
    // unroute
    // once_dialog

    pub async fn wait_for_timeout(&self, timeout: f64) {
        sleep(std::time::Duration::from_millis(timeout as u64)).await
    }
}

#[derive(Clone)]
pub enum Event {
    Close,
    Crash,
    /// Emitted when JavaScript within the page calls one of console API methods, e.g. `console.log` or `console.dir`. Also
    /// emitted if the page throws an error or a warning.
    ///
    /// The arguments passed into `console.log` appear as arguments on the event handler.
    ///
    /// An example of handling `console` event:
    ///
    /// ```js
    /// page.on('console', async msg => {
    ///  for (let i = 0; i < msg.args().length; ++i)
    ///    console.log(`${i}: ${await msg.args()[i].jsonValue()}`);
    /// });
    /// await page.evaluate(() => console.log('hello', 5, {foo: 'bar'}));
    /// ```
    Console(ConsoleMessage),
    /// Emitted when a JavaScript dialog appears, such as `alert`, `prompt`, `confirm` or `beforeunload`. Listener **must**
    /// either [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog - otherwise the page will
    /// [freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking) waiting for the dialog, and
    /// actions like click will never finish.
    ///
    /// > NOTE: When no [`event: Page.dialog`] listeners are present, all dialogs are automatically dismissed.
    Dialog,
    DomContentLoaded,
    /// Emitted when attachment download started. User can access basic file operations on downloaded content via the passed
    /// `Download` instance.
    ///
    /// > NOTE: Browser context **must** be created with the `acceptDownloads` set to `true` when user needs access to the
    /// downloaded content. If `acceptDownloads` is not set, download events are emitted, but the actual download is not
    /// performed and user has no access to the downloaded files.
    Download(Download),
    /// Emitted when a file chooser is supposed to appear, such as after clicking the  `<input type=file>`. Playwright can
    /// respond to it via setting the input files using [`method: FileChooser.setFiles`] that can be uploaded after that.
    ///
    /// ```js
    /// page.on('filechooser', async (fileChooser) => {
    ///  await fileChooser.setFiles('/tmp/myfile.pdf');
    /// });
    /// ```
    // FileChooser(FileChooser),
    FrameAttached(Frame),
    FrameDetached(Frame),
    FrameNavigated(Frame),
    Load,
    PageError,
    /// Emitted when the page opens a new tab or window. This event is emitted in addition to the
    /// [`event: BrowserContext.page`], but only for popups relevant to this page.
    ///
    /// The earliest moment that page is available is when it has navigated to the initial url. For example, when opening a
    /// popup with `window.open('http://example.com')`, this event will fire when the network request to <http://example.com> is
    /// done and its response has started loading in the popup.
    ///
    /// ```js
    /// const [popup] = await Promise.all([
    ///  page.waitForEvent('popup'),
    ///  page.evaluate(() => window.open('https://example.com')),
    /// ]);
    /// console.log(await popup.evaluate('location.href'));
    /// ```
    ///
    /// > NOTE: Use [`method: Page.waitForLoadState`] to wait until the page gets to a particular state (you should not need it
    /// in most cases).
    Popup(Page),
    /// Emitted when a page issues a request. The request object is read-only. In order to intercept and mutate requests, see
    /// [`method: Page.route`] or [`method: BrowserContext.route`].
    Request(Request),
    /// Emitted when a request fails, for example by timing out.
    ///
    /// > NOTE: HTTP Error responses, such as 404 or 503, are still successful responses from HTTP standpoint, so request will
    /// complete with [`event: Page.requestFinished`] event and not with [`event: Page.requestFailed`].
    RequestFailed(Request),
    /// Emitted when a request finishes successfully after downloading the response body. For a successful response, the
    /// sequence of events is `request`, `response` and `requestfinished`.
    RequestFinished(Request),
    /// Emitted when response status and headers are received for a request. For a successful response, the sequence of events
    /// is `request`, `response` and `requestfinished`.
    Response(Response),
    WebSocket(WebSocket),
    Worker(Worker),
    Video(Video)
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let current_event = match self {
            Event::Close => "Close",
            Event::Crash => "Crash",
            Event::Console(_) => "Console",
            Event::Dialog => "Dialog",
            Event::DomContentLoaded => "DomContentLoaded",
            Event::Download(_) => "Download(_)",
            // Event::FileChooser(_) => "FileChooser(_)",
            Event::FrameAttached(_) => "FrameAttached(_)",
            Event::FrameDetached(_) => "FrameDetached(_)",
            Event::FrameNavigated(_) => "FrameNavigated(_)",
            Event::Load => "Load",
            Event::PageError => "PageError",
            Event::Popup(_) => "Popup(_)",
            Event::Request(_) => "Request(_)",
            Event::RequestFailed(_) => "RequestFailed(_)",
            Event::RequestFinished(_) => "RequestFinished(_)",
            Event::Response(_) => "Response(_)",
            Event::WebSocket(_) => "WebSocket(_)",
            Event::Worker(_) => "Worker(_)",
            Event::Video(_) => "Video(_)"
        };
        write!(f, "{}", current_event)
    }
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Event {
        match e {
            Evt::Close => Event::Close,
            Evt::Crash => Event::Crash,
            Evt::Console(x) => Event::Console(ConsoleMessage::new(x)),
            Evt::Dialog => Event::Dialog,
            Evt::Download(x) => Event::Download(Download::new(x)),
            // Evt::FileChooser(x) => Event::FileChooser(x),
            Evt::DomContentLoaded => Event::DomContentLoaded,
            Evt::PageError => Event::PageError,
            Evt::Request(x) => Event::Request(Request::new(x)),
            Evt::Response(x) => Event::Response(Response::new(x)),
            Evt::RequestFailed(x) => Event::RequestFailed(Request::new(x)),
            Evt::RequestFinished(x) => Event::RequestFinished(Request::new(x)),
            Evt::FrameAttached(x) => Event::FrameAttached(Frame::new(x)),
            Evt::FrameDetached(x) => Event::FrameDetached(Frame::new(x)),
            Evt::FrameNavigated(x) => Event::FrameNavigated(Frame::new(x)),
            Evt::Load => Event::Load,
            Evt::Popup(x) => Event::Popup(Page::new(x)),
            Evt::WebSocket(x) => Event::WebSocket(WebSocket::new(x)),
            Evt::Worker(x) => Event::Worker(Worker::new(x)),
            Evt::Video(x) => Event::Video(Video::new(x))
        }
    }
}

impl IsEvent for Event {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Self::Close => EventType::Close,
            Self::Crash => EventType::Crash,
            Self::Console(_) => EventType::Console,
            Self::Dialog => EventType::Dialog,
            Self::Download(_) => EventType::Download,
            // Self::FileChooser(_) => EventType::FileChooser,
            Self::DomContentLoaded => EventType::DomContentLoaded,
            Self::PageError => EventType::PageError,
            Self::Request(_) => EventType::Request,
            Self::Response(_) => EventType::Response,
            Self::RequestFailed(_) => EventType::RequestFailed,
            Self::RequestFinished(_) => EventType::RequestFinished,
            Self::FrameAttached(_) => EventType::FrameAttached,
            Self::FrameDetached(_) => EventType::FrameDetached,
            Self::FrameNavigated(_) => EventType::FrameNavigated,
            Self::Load => EventType::Load,
            Self::Popup(_) => EventType::Popup,
            Self::WebSocket(_) => EventType::WebSocket,
            Self::Worker(_) => EventType::Worker,
            Self::Video(_) => EventType::Video
        }
    }
}

macro_rules! is_checked {
    ($f: ident, $c: meta) => {
        #[$c]
        pub async fn $f(&self, selector: &str, timeout: Option<f64>) -> ArcResult<bool> {
            self.main_frame().$f(selector, timeout).await
        }
    };
}

/// Shorthand of main_frame
impl Page {
    pub async fn query_selector(&self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        self.main_frame().query_selector(selector).await
    }

    pub async fn query_selector_all(&self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        self.main_frame().query_selector_all(selector).await
    }

    pub fn wait_for_selector_builder<'a>(&self, selector: &'a str) -> WaitForSelectorBuilder<'a> {
        self.main_frame().wait_for_selector_builder(selector)
    }

    is_checked! {is_checked, doc = "Errors if the element is not a checkbox or radio input."}
    is_checked! {is_disabled, doc = ""}
    is_checked! {is_editable, doc = ""}
    is_checked! {is_enabled, doc = ""}
    is_checked! {is_hidden, doc = ""}
    is_checked! {is_visible, doc =""}

    pub async fn dispatch_event<T>(
        &self,
        selector: &str,
        r#type: &str,
        event_init: Option<T>
    ) -> ArcResult<()>
    where
        T: Serialize
    {
        // timeout not supported
        self.main_frame()
            .dispatch_event(selector, r#type, event_init)
            .await
    }

    pub async fn evaluate_js_handle<T>(
        &self,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<JsHandle>
    where
        T: Serialize
    {
        self.main_frame().evaluate_js_handle(expression, arg).await
    }

    pub async fn evaluate_element_handle<T>(
        &self,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<ElementHandle>
    where
        T: Serialize
    {
        self.main_frame()
            .evaluate_element_handle(expression, arg)
            .await
    }

    pub async fn eval<U>(&self, expression: &str) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        self.main_frame().eval(expression).await
    }

    pub async fn evaluate<T, U>(&self, expression: &str, arg: T) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        self.main_frame().evaluate(expression, arg).await
    }

    pub async fn evaluate_on_selector<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        self.main_frame()
            .evaluate_on_selector(selector, expression, arg)
            .await
    }

    pub async fn evaluate_on_selector_all<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        self.main_frame()
            .evaluate_on_selector_all(selector, expression, arg)
            .await
    }

    pub fn add_script_tag_builder<'a>(&self, content: &'a str) -> AddScriptTagBuilder<'a, '_, '_> {
        AddScriptTagBuilder::new(self.main_frame_weak(), content)
    }

    pub async fn add_style_tag(
        &self,
        content: &str,
        url: Option<&str>
    ) -> ArcResult<ElementHandle> {
        self.main_frame().add_style_tag(content, url).await
    }

    pub fn url(&self) -> Result<String, Error> { self.main_frame().url() }

    /// Gets the full HTML contents of the page, including the doctype.
    pub async fn content<'a>(&self) -> ArcResult<String> { self.main_frame().content().await }

    pub fn set_content_builder<'a>(&self, html: &'a str) -> SetContentBuilder<'a> {
        self.main_frame().set_content_builder(html)
    }

    /// Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the
    /// last redirect.
    ///
    /// `page.goto` will throw an error if:
    /// - there's an SSL error (e.g. in case of self-signed certificates).
    /// - target URL is invalid.
    /// - the `timeout` is exceeded during navigation.
    /// - the remote server does not respond or is unreachable.
    /// - the main resource failed to load.
    ///
    /// `page.goto` will not throw an error when any valid HTTP status code is returned by the remote server, including 404 "Not
    /// Found" and 500 "Internal Server Error".  The status code for such responses can be retrieved by calling
    /// [`method: Response.status`].
    ///
    /// > NOTE: `page.goto` either throws an error or returns a main resource response. The only exceptions are navigation to
    /// `about:blank` or navigation to the same URL with a different hash, which would succeed and return `null`.
    /// > NOTE: Headless mode doesn't support navigation to a PDF document. See the
    /// [upstream issue](https://bugs.chromium.org/p/chromium/issues/detail?id=761295).
    ///
    /// Shortcut for main frame's [`method: Frame.goto`]
    pub fn goto_builder<'a>(&self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.main_frame_weak(), url)
    }

    // wait_for_load_state

    pub async fn title(&self) -> ArcResult<String> { self.main_frame().title().await }

    pub fn click_builder<'a>(&self, selector: &'a str) -> ClickBuilder<'a> {
        self.main_frame().click_builder(selector)
    }

    pub fn dblclick_builder<'a>(&self, selector: &'a str) -> DblClickBuilder<'a> {
        self.main_frame().dblclick_builder(selector)
    }

    pub fn tap_builder<'a>(&self, selector: &'a str) -> TapBuilder<'a> {
        self.main_frame().tap_builder(selector)
    }

    pub fn fill_builder<'a, 'b>(&self, selector: &'a str, value: &'b str) -> FillBuilder<'a, 'b> {
        self.main_frame().fill_builder(selector, value)
    }

    pub async fn focus(&self, selector: &str, timeout: Option<f64>) -> ArcResult<()> {
        self.main_frame().focus(selector, timeout).await
    }

    pub async fn text_content(
        &self,
        selector: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        self.main_frame().text_content(selector, timeout).await
    }

    pub async fn inner_text(&self, selector: &str, timeout: Option<f64>) -> ArcResult<String> {
        self.main_frame().inner_text(selector, timeout).await
    }

    pub async fn inner_html(&self, selector: &str, timeout: Option<f64>) -> ArcResult<String> {
        self.main_frame().inner_html(selector, timeout).await
    }

    pub async fn get_attribute(
        &self,
        selector: &str,
        name: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        self.main_frame()
            .get_attribute(selector, name, timeout)
            .await
    }

    pub fn hover_builder<'a>(&self, selector: &'a str) -> HoverBuilder<'a> {
        self.main_frame().hover_builder(selector)
    }

    pub fn select_option_builder<'a>(&self, selector: &'a str) -> SelectOptionBuilder<'a> {
        self.main_frame().select_option_builder(selector)
    }

    pub fn set_input_files_builder<'a>(
        &self,
        selector: &'a str,
        file: File
    ) -> SetInputFilesBuilder<'a> {
        self.main_frame().set_input_files_builder(selector, file)
    }

    pub fn type_builer<'a, 'b>(&self, selector: &'a str, text: &'b str) -> TypeBuilder<'a, 'b> {
        self.main_frame().type_builder(selector, text)
    }

    pub fn press_builder<'a, 'b>(&self, selector: &'a str, key: &'b str) -> PressBuilder<'a, 'b> {
        self.main_frame().press_builder(selector, key)
    }

    pub fn check_builder<'a>(&self, selector: &'a str) -> CheckBuilder<'a> {
        self.main_frame().check_builder(selector)
    }

    pub fn uncheck_builder<'a>(&self, selector: &'a str) -> UncheckBuilder<'a> {
        self.main_frame().uncheck_builder(selector)
    }

    pub fn wait_for_function_builder<'a>(&self, expression: &'a str) -> WaitForFunctionBuilder<'a> {
        self.main_frame().wait_for_function_builder(expression)
    }
    // expect_navigation
}

macro_rules! navigation {
    ($t: ident, $f: ident) => {
        pub struct $t {
            inner: Weak<Impl>,
            args: ReloadArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<Impl>) -> Self {
                let args = ReloadArgs::default();
                Self { inner, args }
            }

            pub async fn $f(self) -> ArcResult<Option<Response>> {
                let Self { inner, args } = self;
                let r = upgrade(&inner)?.$f(args).await?;
                Ok(r.map(Response::new))
            }

            setter! {
                timeout: Option<f64>,
                /// When to consider operation succeeded, defaults to `load`. Events can be either:
                /// - `'domcontentloaded'` - consider operation to be finished when the `DOMContentLoaded` event is fired.
                /// - `'load'` - consider operation to be finished when the `load` event is fired.
                /// - `'networkidle'` - consider operation to be finished when there are no network connections for at least `500` ms.
                wait_until: Option<LifecycleEvent>
            }
        }
    };
}

navigation!(ReloadBuilder, reload);
navigation!(GoBackBuilder, go_back);
navigation!(GoForwardBuilder, go_forward);

pub struct PdfBuilder<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    inner: Weak<Impl>,
    args: PdfArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> PdfBuilder<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = PdfArgs::default();
        Self { inner, args }
    }

    pub async fn pdf(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.pdf(args).await?;
        Ok(())
    }

    setter! {
        /// Scale of the webpage rendering. Defaults to `1`. Scale amount must be between 0.1 and 2.
        scale: Option<f64>,
        /// Display header and footer. Defaults to `false`.
        display_header_footer: Option<bool>,
        /// HTML template for the print header. Should be valid HTML markup with following classes used to inject printing values
        /// into them:
        /// - `'date'` formatted print date
        /// - `'title'` document title
        /// - `'url'` document location
        /// - `'pageNumber'` current page number
        /// - `'totalPages'` total pages in the document
        header_template: Option<&'a str>,
        /// HTML template for the print footer. Should use the same format as the `headerTemplate`.
        footer_template: Option<&'b str>,
        /// Print background graphics. Defaults to `false`.
        print_background: Option<bool>,
        /// Paper orientation. Defaults to `false`.
        landscape: Option<bool>,
        /// Paper ranges to print, e.g., '1-5, 8, 11-13'. Defaults to the empty string, which means print all pages.
        page_ranges: Option<&'c str>,
        /// Paper format. If set, takes priority over `width` or `height` options. Defaults to 'Letter'.
        format: Option<&'d str>,
        /// Paper width, accepts values labeled with units.
        width: Option<Length<'e>>,
        /// Paper height, accepts values labeled with units.
        height: Option<Length<'f>>,
        /// Give any CSS `@page` size declared in the page priority over what is declared in `width` and `height` or `format`
        /// options. Defaults to `false`, which will scale the content to fit the paper size.
        prefer_css_page_size: Option<bool>,
        /// Paper margins, defaults to none.
        margin: Option<PdfMargins<'g, 'h, 'i, 'j>>,
        /// The file path to save the PDF to. If `path` is a relative path, then it is resolved relative to the current working
        /// directory. If no path is provided, the PDF won't be saved to the disk.
        path: Option<PathBuf>
    }
}

pub struct ScreenshotBuilder {
    inner: Weak<Impl>,
    args: ScreenshotArgs
}

impl ScreenshotBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = ScreenshotArgs::default();
        Self { inner, args }
    }

    pub async fn screenshot(self) -> ArcResult<Vec<u8>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.screenshot(args).await
    }

    pub fn r#type(mut self, x: ScreenshotType) -> Self {
        self.args.r#type = Some(x);
        self
    }

    setter! {
        /// An object which specifies clipping of the resulting image. Should have the following fields:
        clip: Option<FloatRect>,
        /// When true, takes a screenshot of the full scrollable page, instead of the currently visible viewport. Defaults to
        /// `false`.
        full_page: Option<bool>,
        /// Hides default white background and allows capturing screenshots with transparency. Not applicable to `jpeg` images.
        /// Defaults to `false`.
        omit_background: Option<bool>,
        quality: Option<i32>,
        /// Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by
        /// using the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods.
        timeout: Option<f64>,
        /// The file path to save the image to. The screenshot type will be inferred from file extension. If `path` is a relative
        /// path, then it is resolved relative to the current working directory. If no path is provided, the image won't be saved to
        /// the disk.
        path: Option<PathBuf>
    }

    pub fn clear_type(mut self) -> Self {
        self.args.r#type = None;
        self
    }
}

pub struct EmulateMediaBuilder {
    inner: Weak<Impl>,
    args: EmulateMediaArgs
}

impl EmulateMediaBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = EmulateMediaArgs::default();
        Self { inner, args }
    }

    pub async fn emulate_media(self) -> ArcResult<()> {
        let Self { inner, args } = self;
        upgrade(&inner)?.emulate_media(args).await
    }

    setter! {
        /// Emulates `'prefers-colors-scheme'` media feature, supported values are `'light'`, `'dark'`, `'no-preference'`.
        // NOTE: Not implemented passing `null` disables color scheme emulation
        color_scheme: Option<ColorScheme>,
        /// Changes the CSS media type of the page. The only allowed values are `'screen'`, `'print'` and `null`. Passing `null`
        /// disables CSS media emulation.
        media: Option<Media>
    }
}
