use crate::{
    api::Frame,
    imp::{core::*, element_handle::ElementHandle as Impl, prelude::*}
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

    pub async fn owner_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.owner_frame().await?.map(Frame::new))
    }

    pub async fn content_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.content_frame().await?.map(Frame::new))
    }

    pub async fn get_attribute(&self, name: &str) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.get_attribute(name).await
    }
}
