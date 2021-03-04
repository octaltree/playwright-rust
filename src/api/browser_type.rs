use crate::{
    browser::Browser,
    browser_context::BrowserContext,
    imp::{self, core::*, prelude::*},
    Error
};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Rweak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Rweak<imp::browser_type::BrowserType>) -> Self { Self { inner } }

    pub fn name(&self) -> String { upgrade(&self.inner).unwrap().name().into() }

    pub fn executable(&self) -> PathBuf { upgrade(&self.inner).unwrap().executable().into() }

    pub async fn launch(&mut self) -> Result<Browser, Error> {
        unimplemented!();
    }

    pub async fn launch_persistent_context(&mut self) -> Result<Browser, Error> {
        unimplemented!();
    }
}
