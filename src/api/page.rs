pub use crate::api::frame::{
    ClickBuilder, DblClickBuilder, GotoBuilder, HoverBuilder, PressBuilder, SetContentBuilder,
    TypeBuilder, WaitForSelectorBuilder
};
use crate::{
    api::{
        accessibility::Accessibility, browser_context::BrowserContext,
        element_handle::ElementHandle, frame::Frame, input_device::*, response::Response,
        video::Video, worker::Worker, Keyboard, TouchScreen
    },
    imp::{
        core::*,
        frame::Frame as FrameImpl,
        page::{Page as Impl, ReloadArgs},
        prelude::*,
        utils::{DocumentLoadState, Viewport}
    },
    Error
};
use std::time::Duration;

pub struct Page {
    inner: Weak<Impl>,
    pub keyboard: Keyboard,
    pub touch_screen: TouchScreen,
    pub mouse: Mouse,
    pub accessibility: Accessibility
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

    fn main_frame_weak(&self) -> Weak<FrameImpl> {
        weak_and_then(&self.inner, |rc| rc.main_frame())
    }

    pub fn main_frame(&self) -> Frame { Frame::new(self.main_frame_weak()) }

    pub fn reload_builder(&mut self) -> ReloadBuilder { ReloadBuilder::new(self.inner.clone()) }
    pub fn go_back_builder(&mut self) -> GoBackBuilder { GoBackBuilder::new(self.inner.clone()) }
    pub fn go_forward_builder(&mut self) -> GoForwardBuilder {
        GoForwardBuilder::new(self.inner.clone())
    }

    ///// Video object associated with this page.
    // fn video(&self) -> Option<Video> { unimplemented!() }

    ///// Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`.
    // async fn opener(&mut self) -> Option<Page> { unimplemented!() }

    ///// Returns frame matching the specified criteria. Either `name` or `url` must be specified.
    // fn frame(&self) -> Option<Frame> { unimplemented!() }

    ///// Returns whether the element is checked. Throws if the element is not a checkbox or radio input.
    // async fn is_checked(self) -> Option<ElementHandle> { unimplemented!() }

    ///// Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled).
    // async fn is_disabled(self) -> Option<ElementHandle> { unimplemented!() }

    ///// Returns whether the element is [editable](./actionability.md#editable).
    // async fn is_editable(self) -> Option<ElementHandle> { unimplemented!() }

    //// TODO
}

macro_rules! is_checked {
    ($f: ident, $c: meta) => {
        #[$c]
        pub async fn $f(&mut self, selector: &str, timeout: Option<f64>) -> ArcResult<bool> {
            self.main_frame().$f(selector, timeout).await
        }
    };
}

/// Shorthand of main_frame
impl Page {
    pub async fn query_selector(&mut self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        self.main_frame().query_selector(selector).await
    }

    pub async fn query_selector_all(&mut self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        self.main_frame().query_selector_all(selector).await
    }

    pub fn wait_for_selector_builder<'a>(
        &mut self,
        selector: &'a str
    ) -> WaitForSelectorBuilder<'a> {
        self.main_frame().wait_for_selector_builder(selector)
    }

    is_checked! {is_checked, doc = "Errors if the element is not a checkbox or radio input."}
    is_checked! {is_disabled, doc = ""}
    is_checked! {is_editable, doc = ""}
    is_checked! {is_enabled, doc = ""}
    is_checked! {is_hidden, doc = ""}
    is_checked! {is_visible, doc =""}
    // dispatch_event
    // evaluate
    // evaluate_handle
    // eval_on_selector
    // eval_on_selector_all
    // add_script_tag
    // add_style_tag
    // url
    // content

    pub fn set_content_builder<'a>(&mut self, html: &'a str) -> SetContentBuilder<'a> {
        self.main_frame().set_content_builder(html)
    }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.main_frame_weak(), url)
    }

    // wait_for_load_state

    pub async fn title(&mut self) -> ArcResult<String> { self.main_frame().title().await }

    pub fn click_builder<'a>(&mut self, selector: &'a str) -> ClickBuilder<'a> {
        self.main_frame().click_builder(selector)
    }

    pub fn dblclick_builder<'a>(&mut self, selector: &'a str) -> DblClickBuilder<'a> {
        self.main_frame().dblclick_builder(selector)
    }

    // tap
    // fill
    // focus
    // text_content
    // inner_text
    // inner_html
    // get_attribute

    pub fn hover<'a>(&mut self, selector: &'a str) -> HoverBuilder<'a> {
        self.main_frame().hover_builder(selector)
    }

    // select_option
    // set_input_files
    // type

    pub fn r#type<'a, 'b>(&mut self, selector: &'a str, text: &'b str) -> TypeBuilder<'a, 'b> {
        self.main_frame().type_builder(selector, text)
    }

    pub fn press<'a, 'b>(&mut self, selector: &'a str, key: &'b str) -> PressBuilder<'a, 'b> {
        self.main_frame().press_builder(selector, key)
    }

    // check
    // uncheck
    // wait_for_timeout
    // wait_for_function
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
