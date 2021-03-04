use crate::imp::{core::*, prelude::*, selectors};

#[derive(Debug, Clone)]
pub struct Selectors {
    inner: Weak<selectors::Selectors>
}

impl Selectors {
    pub(crate) fn new(inner: Weak<selectors::Selectors>) -> Self { Self { inner } }

    pub async fn register(
        &mut self,
        name: &str,
        script: &str,
        content_script: bool
    ) -> Result<(), Arc<ConnectionError>> {
        let inner = upgrade(&self.inner)?;
        inner.register(name, script, content_script).await
    }
}
