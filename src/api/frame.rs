pub use crate::imp::{
    frame::FrameState,
    utils::{KeyboardModifier, MouseButton, Position}
};
use crate::{
    api::{ElementHandle, Response},
    imp::{
        core::*,
        frame::{ClickArgs, Frame as Impl, GotoArgs, WaitForSelectorArgs},
        prelude::*,
        utils::DocumentLoadState
    }
};

pub struct Frame {
    inner: Weak<Impl>
}

impl Frame {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.inner.clone(), url)
    }

    pub fn clicker<'a>(&mut self, selector: &'a str) -> Clicker<'a> {
        Clicker::new(self.inner.clone(), selector)
    }

    pub fn dblclicker<'a>(&mut self, selector: &'a str) -> DblClicker<'a> {
        DblClicker::new(self.inner.clone(), selector)
    }

    pub async fn query_selector(&mut self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        Ok(upgrade(&self.inner)?
            .query_selector(selector)
            .await?
            .map(ElementHandle::new))
    }

    pub async fn query_selector_all(&mut self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        let es = upgrade(&self.inner)?.query_selector_all(selector).await?;
        Ok(es.into_iter().map(ElementHandle::new).collect())
    }

    pub async fn frame_element(&mut self) -> ArcResult<ElementHandle> {
        Ok(ElementHandle::new(
            upgrade(&self.inner)?.frame_element().await?
        ))
    }

    pub fn wait_for_selector_builder<'a>(
        &mut self,
        selector: &'a str
    ) -> WaitForSelectorBuilder<'a> {
        WaitForSelectorBuilder::new(self.inner.clone(), selector)
    }
}

pub struct GotoBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: GotoArgs<'a, 'b>
}

impl<'a, 'b> GotoBuilder<'a, 'b> {
    pub(crate) fn new(inner: Weak<Impl>, url: &'a str) -> Self {
        let args = GotoArgs::new(url);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<Option<Response>, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.goto(args).await?;
        Ok(r.map(Response::new))
    }

    optional_setter!(
        timeout, f64;
        wait_until, DocumentLoadState;
        referer, &'b str);
}

macro_rules! clicker {
    ($t: ident, $f: ident) => {
        pub struct $t<'a> {
            inner: Weak<Impl>,
            args: ClickArgs<'a>
        }

        impl<'a> $t<'a> {
            pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
                let args = ClickArgs::new(selector);
                Self { inner, args }
            }

            pub async fn $f(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$f(args).await?;
                Ok(())
            }

            optional_setter!(
                modifiers, Vec<KeyboardModifier>;
                position, Position;
                delay, f64;
                button, MouseButton;
                click_count, i32;
                timeout, f64;
                force, bool;
                no_wait_after, bool);
        }
    }
}

clicker!(Clicker, click);
clicker!(DblClicker, dblclick);

pub struct WaitForSelectorBuilder<'a> {
    inner: Weak<Impl>,
    args: WaitForSelectorArgs<'a>
}

impl<'a> WaitForSelectorBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = WaitForSelectorArgs::new(selector);
        Self { inner, args }
    }

    pub async fn wait_for_selector(self) -> Result<Option<ElementHandle>, Arc<Error>> {
        let Self { inner, args } = self;
        let e = upgrade(&inner)?.wait_for_selector(args).await?;
        Ok(e.map(ElementHandle::new))
    }

    optional_setter!(
        timeout, f64;
        state, FrameState);
}
