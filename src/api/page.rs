pub use crate::{
    api::{
        frame::{
            AddScriptTagBuilder, CheckBuilder, ClickBuilder, DblClickBuilder, FillBuilder,
            GotoBuilder, HoverBuilder, PressBuilder, SelectOptionBuilder, SetContentBuilder,
            SetInputFilesBuilder, TapBuilder, TypeBuilder, UncheckBuilder, WaitForSelectorBuilder
        },
        JsHandle, Request
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
        utils::{
            ColorScheme, DocumentLoadState, File, FloatRect, Length, PdfMargins, ScreenshotType,
            Viewport
        }
    },
    Error
};

#[derive(Debug)]
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

    pub fn main_frame(&self) -> Frame { Frame::new(self.main_frame_weak()) }

    pub fn frames(&self) -> Result<Vec<Frame>, Error> {
        Ok(upgrade(&self.inner)?
            .frames()
            .into_iter()
            .map(Frame::new)
            .collect())
    }

    pub fn workers(&self) -> Result<Vec<Worker>, Error> {
        Ok(upgrade(&self.inner)?
            .workers()
            .into_iter()
            .map(Worker::new)
            .collect())
    }

    pub fn reload_builder(&self) -> ReloadBuilder { ReloadBuilder::new(self.inner.clone()) }
    pub fn go_back_builder(&self) -> GoBackBuilder { GoBackBuilder::new(self.inner.clone()) }
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

    pub async fn set_viewport_size(&self, viewport_size: Viewport) -> ArcResult<()> {
        upgrade(&self.inner)?.set_viewport_size(viewport_size).await
    }

    ///// Video object associated with this page.
    // fn video(&self) -> Option<Video> { unimplemented!() }

    ///// Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`.
    // async fn opener(& self) -> Option<Page> { unimplemented!() }

    ///// Returns frame matching the specified criteria. Either `name` or `url` must be specified.
    // fn frame(&self) -> Option<Frame> { unimplemented!() }

    pub async fn bring_to_front(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.bring_to_front().await
    }

    pub async fn add_init_script(&self, source: &str) -> ArcResult<()> {
        upgrade(&self.inner)?.add_init_script(source).await
    }

    pub fn pdf_builder(&self) -> PdfBuilder<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        PdfBuilder::new(self.inner.clone())
    }

    /// All temporary pages will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
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

    pub fn emulate_media(&self) -> EmulateMediaBuilder {
        EmulateMediaBuilder::new(self.inner.clone())
    }

    pub async fn opener(&self) -> ArcResult<Option<Page>> {
        Ok(upgrade(&self.inner)?.opener().await?.map(Page::new))
    }

    pub async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        upgrade(&self.inner)?.set_extra_http_headers(headers).await
    }

    pub async fn expect_event(&self, evt: EventType) -> Result<Event, Error> {
        upgrade(&self.inner)?
            .expect_event(evt)
            .await
            .map(Event::from)
    }

    subscribe_event! {}
}

pub enum Event {
    Close,
    Crash,
    Console(ConsoleMessage),
    Dialog,
    Download,
    FileChooser,
    DOMContentLoaded,
    PageError,
    Request(Request),
    Response(Response),
    RequestFailed(Request),
    RequestFinished(Request),
    FrameAttached(Frame),
    FrameDetached(Frame),
    FrameNavigated(Frame),
    Load,
    Popup(Page),
    WebSocket(WebSocket),
    Worker(Worker)
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Event {
        match e {
            Evt::Close => Event::Close,
            Evt::Crash => Event::Crash,
            Evt::Console(x) => Event::Console(ConsoleMessage::new(x)),
            Evt::Dialog => Event::Dialog,
            Evt::Download => Event::Download,
            Evt::FileChooser => Event::FileChooser,
            Evt::DOMContentLoaded => Event::DOMContentLoaded,
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
            Evt::Worker(x) => Event::Worker(Worker::new(x))
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
        self.main_frame()
            .dispatch_event(selector, r#type, event_init)
            .await
    }

    pub async fn eval_handle(&self, expression: &str) -> ArcResult<JsHandle> {
        self.main_frame().eval_handle(expression).await
    }

    pub async fn evaluate_handle<T>(&self, expression: &str, arg: Option<T>) -> ArcResult<JsHandle>
    where
        T: Serialize
    {
        self.main_frame().evaluate_handle(expression, arg).await
    }

    pub async fn eval<U>(&self, expression: &str) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        self.main_frame().eval(expression).await
    }

    pub async fn evaluate<T, U>(&self, expression: &str, arg: Option<T>) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        self.main_frame().evaluate(expression, arg).await
    }

    pub async fn eval_on_selector<T, U>(
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
            .eval_on_selector(selector, expression, arg)
            .await
    }

    pub async fn eval_on_selector_all<T, U>(
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
            .eval_on_selector_all(selector, expression, arg)
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

    pub async fn content<'a>(&self) -> ArcResult<String> { self.main_frame().content().await }

    pub fn set_content_builder<'a>(&self, html: &'a str) -> SetContentBuilder<'a> {
        self.main_frame().set_content_builder(html)
    }

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

    pub async fn wait_for_timeout(&self, timeout: f64) {
        sleep(std::time::Duration::from_millis(timeout as u64)).await
    }

    pub async fn wait_for_function_builder<'a>(
        &self,
        expression: &'a str
    ) -> WaitForSelectorBuilder<'a> {
        self.main_frame().wait_for_selector_builder(expression)
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

            optional_setter!(
                timeout, f64;
                wait_until, DocumentLoadState);
        }
    }
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

    optional_setter!(
        scale, f64;
        display_header_footer, bool;
        header_template, &'a str;
        footer_template, &'b str;
        print_background, bool;
        landscape, bool;
        page_ranges, &'c str;
        format, &'d str;
        width, Length<'e>;
        height, Length<'f>;
        prefer_css_page_size, bool;
        margin, PdfMargins<'g,'h,'i,'j>);
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

    optional_setter!(
        timeout, f64;
        quality, i32;
        omit_background, bool;
        full_page, bool;
        clip, FloatRect);

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

    optional_setter!(
        media, Media;
        color_scheme, ColorScheme);
}
