use crate::imp::{core::*, js_handle::JsHandle as Impl, prelude::*};

pub struct JsHandle {
    inner: Weak<Impl>
}

impl JsHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub async fn get_property(&mut self, name: &str) -> ArcResult<JsHandle> {
        upgrade(&self.inner)?
            .get_property(name)
            .await
            .map(JsHandle::new)
    }

    pub async fn get_properties(&mut self, name: &str) -> ArcResult<HashMap<String, JsHandle>> {
        let m = upgrade(&self.inner)?.get_properties().await?;
        Ok(m.into_iter().map(|(k, v)| (k, JsHandle::new(v))).collect())
    }

    pub async fn dispose(&mut self) -> ArcResult<()> { upgrade(&self.inner)?.dispose().await }
}
