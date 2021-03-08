use crate::{
    api::Frame,
    imp::{
        core::*,
        element_handle::{ClickArgs, ElementHandle as Impl, HoverArgs},
        prelude::*,
        utils::{KeyboardModifier, MouseButton, Position}
    }
};

pub struct ElementHandle {
    inner: Weak<Impl>
}

macro_rules! is_checked {
    ($f: ident) => {
        pub async fn $f(&mut self) -> ArcResult<bool> { upgrade(&self.inner)?.$f().await }
    };
}

impl ElementHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

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

    pub async fn inner_text(&mut self) -> ArcResult<String> {
        upgrade(&self.inner)?.inner_text().await
    }

    pub async fn inner_html(&mut self) -> ArcResult<String> {
        upgrade(&self.inner)?.inner_html().await
    }

    is_checked! {is_checked}
    is_checked! {is_disabled}
    is_checked! {is_editable}
    is_checked! {is_enabled}
    is_checked! {is_hidden}
    is_checked! {is_visible}

    pub async fn owner_frame(&mut self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.owner_frame().await?.map(Frame::new))
    }

    pub async fn content_frame(&mut self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.content_frame().await?.map(Frame::new))
    }

    pub async fn get_attribute(&mut self, name: &str) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.get_attribute(name).await
    }

    pub async fn text_content(&mut self) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.text_content().await
    }

    pub async fn hover(&mut self) -> HoverBuilder { HoverBuilder::new(self.inner.clone()) }

    pub async fn click_builder(&mut self) -> ClickBuilder { ClickBuilder::new(self.inner.clone()) }

    pub async fn dblclick_builder(&mut self) -> DblClickBuilder {
        DblClickBuilder::new(self.inner.clone())
    }
}

pub struct HoverBuilder {
    inner: Weak<Impl>,
    args: HoverArgs
}

impl HoverBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = HoverArgs::default();
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

macro_rules! clicker {
    ($t: ident, $f: ident) => {
        pub struct $t {
            inner: Weak<Impl>,
            args: ClickArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<Impl>) -> Self {
                let args = ClickArgs::default();
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
