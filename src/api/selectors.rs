use crate::imp::{core::*, prelude::*, selectors::Selectors as Impl};

#[derive(Debug, Clone)]
pub struct Selectors {
    inner: Weak<Impl>
}

impl Selectors {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub async fn register(
        &mut self,
        name: &str,
        script: &str,
        content_script: bool
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.register(name, script, content_script).await
    }
}
