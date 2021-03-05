use crate::{
    api::{browser::Browser, browser_context::BrowserContext},
    imp::{self, core::*, prelude::*},
    Error
};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Weak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Weak<imp::browser_type::BrowserType>) -> Self { Self { inner } }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn name(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.name().into()) }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn executable(&self) -> Result<PathBuf, Error> {
        Ok(upgrade(&self.inner)?.executable().into())
    }

    // pub async fn launch(&mut self, args: LaunchArgs<'_, '_, '_>) -> Result<Browser, Error> {
    //    unimplemented!();
    //}

    // async fn launch_persistent_context(&mut self) -> Result<Browser, Error> {
    //    unimplemented!();
    //}
}
