use crate::imp::{self, core::*, prelude::*};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Rweak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Rweak<imp::browser_type::BrowserType>) -> Self { Self { inner } }

    pub fn name(&self) -> String { upgrade(&self.inner).unwrap().name().into() }

    pub fn executable(&self) -> PathBuf { upgrade(&self.inner).unwrap().executable().into() }
}
