pub use crate::api::frame::GotoBuilder;
use crate::{
    api::{
        accessibility::Accessibility, browser_context::BrowserContext,
        element_handle::ElementHandle, frame::Frame, input_device::*, video::Video, worker::Worker
    },
    imp::{self, core::*, page::Page as Impl, prelude::*, utils::Viewport},
    Error
};
use std::time::Duration;

pub struct Page {
    inner: Weak<Impl>
}

impl Page {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn main_frame(&self) -> Frame {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        Frame::new(inner)
    }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        let inner = weak_and_then(&self.inner, |rc| rc.main_frame());
        GotoBuilder::new(inner, url)
    }

    fn accessibility(&self) -> Accessibility { unimplemented!() }

    fn keyboard(&self) -> Keyboard { unimplemented!() }

    fn mouse(&self) -> Mouse { unimplemented!() }

    fn touchscreen(&self) -> TouchScreen { unimplemented!() }

    fn context(&self) -> BrowserContext { unimplemented!() }

    fn frames(&self) -> Vec<Frame> { unimplemented!() }

    fn url(&self) -> String { unimplemented!() }

    fn viewport_size(&self) -> Viewport { unimplemented!() }

    fn workers(&self) -> Vec<Worker> { unimplemented!() }

    /// Video object associated with this page.
    fn video(&self) -> Option<Video> { unimplemented!() }

    /// Returns the opener for popup pages and `null` for others. If the opener has been closed already the returns `null`.
    async fn opener(&mut self) -> Option<Page> { unimplemented!() }

    /// Returns frame matching the specified criteria. Either `name` or `url` must be specified.
    fn frame(&self) -> Option<Frame> { unimplemented!() }

    async fn set_default_navigation_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    async fn set_default_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    async fn query_selector(self, selector: &str) -> Option<ElementHandle> { unimplemented!() }

    async fn query_selector_all(self, selector: &str) -> Vec<ElementHandle> { unimplemented!() }

    async fn wait_for_selector(self) -> Option<ElementHandle> { unimplemented!() }

    /// Returns whether the element is checked. Throws if the element is not a checkbox or radio input.
    async fn is_checked(self) -> Option<ElementHandle> { unimplemented!() }

    /// Returns whether the element is disabled, the opposite of [enabled](./actionability.md#enabled).
    async fn is_disabled(self) -> Option<ElementHandle> { unimplemented!() }

    /// Returns whether the element is [editable](./actionability.md#editable).
    async fn is_editable(self) -> Option<ElementHandle> { unimplemented!() }

    // TODO
}
