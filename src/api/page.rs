pub use crate::{
    api::frame::{Clicker, DblClicker, GotoBuilder, PressBuilder, TypeBuilder},
    imp::utils::DocumentLoadState
};
use crate::{
    api::{
        accessibility::Accessibility, browser_context::BrowserContext,
        element_handle::ElementHandle, frame::Frame, input_device::*, response::Response,
        video::Video, worker::Worker, Keyboard, TouchScreen
    },
    imp::{
        core::*,
        page::{Page as Impl, ReloadArgs},
        prelude::*,
        utils::Viewport
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

    pub fn main_frame(&self) -> Frame {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        Frame::new(inner)
    }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        GotoBuilder::new(inner, url)
    }

    pub fn reload_builder(&mut self) -> ReloadBuilder { ReloadBuilder::new(self.inner.clone()) }
    pub fn go_back_builder(&mut self) -> GoBackBuilder { GoBackBuilder::new(self.inner.clone()) }
    pub fn go_forward_builder(&mut self) -> GoForwardBuilder {
        GoForwardBuilder::new(self.inner.clone())
    }

    pub fn clicker<'a>(&mut self, selector: &'a str) -> Clicker<'a> {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        Clicker::new(inner, selector)
    }

    pub fn dblclicker<'a>(&mut self, selector: &'a str) -> DblClicker<'a> {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        DblClicker::new(inner, selector)
    }

    pub async fn query_selector(&mut self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        self.main_frame().query_selector(selector).await
    }

    pub async fn query_selector_all(&mut self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        self.main_frame().query_selector_all(selector).await
    }

    pub fn r#type<'a, 'b>(&self, selector: &'a str, text: &'b str) -> TypeBuilder<'a, 'b> {
        self.main_frame().r#type(selector, text)
    }

    pub fn press<'a, 'b>(&self, selector: &'a str, key: &'b str) -> PressBuilder<'a, 'b> {
        self.main_frame().press(selector, key)
    }

    // fn accessibility(&self) -> Accessibility { unimplemented!() }

    // fn context(&self) -> BrowserContext { unimplemented!() }

    // fn frames(&self) -> Vec<Frame> { unimplemented!() }

    // fn url(&self) -> String { unimplemented!() }

    // fn viewport_size(&self) -> Viewport { unimplemented!() }

    // fn workers(&self) -> Vec<Worker> { unimplemented!() }

    ///// Video object associated with this page.
    // fn video(&self) -> Option<Video> { unimplemented!() }

    ///// Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`.
    // async fn opener(&mut self) -> Option<Page> { unimplemented!() }

    ///// Returns frame matching the specified criteria. Either `name` or `url` must be specified.
    // fn frame(&self) -> Option<Frame> { unimplemented!() }

    // async fn set_default_navigation_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
    //    unimplemented!()
    //}

    // async fn set_default_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
    //    unimplemented!()
    //}

    // async fn query_selector(self, selector: &str) -> Option<ElementHandle> { unimplemented!() }

    // async fn query_selector_all(self, selector: &str) -> Vec<ElementHandle> { unimplemented!() }

    // async fn wait_for_selector(self) -> Option<ElementHandle> { unimplemented!() }

    ///// Returns whether the element is checked. Throws if the element is not a checkbox or radio input.
    // async fn is_checked(self) -> Option<ElementHandle> { unimplemented!() }

    ///// Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled).
    // async fn is_disabled(self) -> Option<ElementHandle> { unimplemented!() }

    ///// Returns whether the element is [editable](./actionability.md#editable).
    // async fn is_editable(self) -> Option<ElementHandle> { unimplemented!() }

    //// TODO
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
