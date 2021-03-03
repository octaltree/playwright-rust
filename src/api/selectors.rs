use crate::imp::{core::*, prelude::*, selectors};

#[derive(Debug, Clone)]
pub struct Selectors {
    inner: Rweak<selectors::Selectors>
}

impl Selectors {
    pub(crate) fn new(inner: Rweak<selectors::Selectors>) -> Self { Self { inner } }

    pub async fn register(
        &self,
        name: &str,
        script: &str,
        content_script: bool
    ) -> Result<(), Rc<ConnectionError>> {
        let inner = upgrade(&self.inner)?;
        inner.register(name, script, content_script).await
    }
}
