pub use crate::imp::frame::FrameState;
use crate::{
    api::{ElementHandle, Response},
    imp::{
        core::*,
        frame::{
            ClickArgs, FillArgs, Frame as Impl, GotoArgs, HoverArgs, PressArgs, SetContentArgs,
            TapArgs, TypeArgs, WaitForSelectorArgs
        },
        prelude::*,
        utils::{DocumentLoadState, KeyboardModifier, MouseButton, Position}
    }
};

pub struct Frame {
    inner: Weak<Impl>
}

macro_rules! is_checked {
    ($f: ident) => {
        pub async fn $f(&mut self, selector: &str, timeout: Option<f64>) -> ArcResult<bool> {
            upgrade(&self.inner)?.$f(selector, timeout).await
        }
    };
}

impl Frame {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn goto_builder<'a>(&mut self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.inner.clone(), url)
    }

    pub fn click_builder<'a>(&mut self, selector: &'a str) -> ClickBuilder<'a> {
        ClickBuilder::new(self.inner.clone(), selector)
    }

    pub fn dblclick_builder<'a>(&mut self, selector: &'a str) -> DblClickBuilder<'a> {
        DblClickBuilder::new(self.inner.clone(), selector)
    }

    pub fn tap_builder<'a>(&mut self, selector: &'a str) -> TapBuilder<'a> {
        TapBuilder::new(self.inner.clone(), selector)
    }

    pub fn fill_builder<'a, 'b>(
        &mut self,
        selector: &'a str,
        value: &'b str
    ) -> FillBuilder<'a, 'b> {
        FillBuilder::new(self.inner.clone(), selector, value)
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

    pub async fn title(&mut self) -> ArcResult<String> { upgrade(&self.inner)?.title().await }

    pub fn type_builder<'a, 'b>(
        &mut self,
        selector: &'a str,
        text: &'b str
    ) -> TypeBuilder<'a, 'b> {
        TypeBuilder::new(self.inner.clone(), selector, text)
    }

    pub fn press_builder<'a, 'b>(
        &mut self,
        selector: &'a str,
        key: &'b str
    ) -> PressBuilder<'a, 'b> {
        PressBuilder::new(self.inner.clone(), selector, key)
    }

    pub fn hover_builder<'a>(&mut self, selector: &'a str) -> HoverBuilder<'a> {
        HoverBuilder::new(self.inner.clone(), selector)
    }

    is_checked! {is_checked}
    is_checked! {is_disabled}
    is_checked! {is_editable}
    is_checked! {is_enabled}
    is_checked! {is_hidden}
    is_checked! {is_visible}

    pub async fn content<'a>(&mut self) -> ArcResult<String> {
        upgrade(&self.inner)?.content().await
    }

    pub fn set_content_builder<'a>(&mut self, html: &'a str) -> SetContentBuilder<'a> {
        SetContentBuilder::new(self.inner.clone(), html)
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

clicker!(ClickBuilder, click);
clicker!(DblClickBuilder, dblclick);

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

macro_rules! type_builder {
    ($t: ident, $a: ident, $f: ident, $m: ident) => {
        pub struct $t<'a, 'b> {
            inner: Weak<Impl>,
            args: $a<'a, 'b>
        }

        impl<'a, 'b> $t<'a, 'b> {
            pub(crate) fn new(inner: Weak<Impl>, selector: &'a str, $f: &'b str) -> Self {
                let args = $a::new(selector, $f);
                Self { inner, args }
            }

            pub async fn $m(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$m(args).await?;
                Ok(())
            }

            optional_setter!(
                delay, f64;
                timeout, f64;
                no_wait_after, bool);
        }
    }
}

type_builder!(TypeBuilder, TypeArgs, text, r#type);
type_builder!(PressBuilder, PressArgs, key, press);

pub struct HoverBuilder<'a> {
    inner: Weak<Impl>,
    args: HoverArgs<'a>
}

impl<'a> HoverBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = HoverArgs::new(selector);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.hover(args).await
    }

    optional_setter!(
        modifiers, Vec<KeyboardModifier>;
        position, Position;
        timeout, f64;
        force, bool);
}

pub struct SetContentBuilder<'a> {
    inner: Weak<Impl>,
    args: SetContentArgs<'a>
}

impl<'a> SetContentBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, html: &'a str) -> Self {
        let args = SetContentArgs::new(html);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.set_content(args).await
    }

    optional_setter!(
        timeout, f64;
        wait_until, DocumentLoadState);
}

pub struct TapBuilder<'a> {
    inner: Weak<Impl>,
    args: TapArgs<'a>
}

impl<'a> TapBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = TapArgs::new(selector);
        Self { inner, args }
    }

    pub async fn tap(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.tap(args).await?;
        Ok(())
    }

    optional_setter!(
        modifiers, Vec<KeyboardModifier>;
        position, Position;
        timeout, f64;
        force, bool;
        no_wait_after, bool);
}

pub struct FillBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: FillArgs<'a, 'b>
}

impl<'a, 'b> FillBuilder<'a, 'b> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str, value: &'b str) -> Self {
        let args = FillArgs::new(selector, value);
        Self { inner, args }
    }

    pub async fn fill(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.fill(args).await?;
        Ok(())
    }

    optional_setter!(
        timeout, f64;
        no_wait_after, bool);
}
