use crate::{
    api::{browser::Browser, page::Page},
    imp::{
        browser_context::BrowserContext as Impl,
        core::*,
        prelude::*,
        utils::{Cookie, StorageState}
    },
    Error
};
use std::time::Duration;

pub struct BrowserContext {
    inner: Weak<Impl>
}

impl BrowserContext {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    fn pages(&self) -> Result<Vec<Page>, Error> {
        Ok(upgrade(&self.inner)?
            .pages()
            .iter()
            .cloned()
            .map(Page::new)
            .collect())
    }

    pub async fn new_page(&mut self) -> Result<Page, Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        Ok(Page::new(inner.new_page().await?))
    }

    /// Returns the browser instance of the context. If it was launched as a persistent context null gets returned.
    fn browser(&self) -> Option<Browser> { unimplemented!() }

    async fn set_default_navigation_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    async fn set_default_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
        unimplemented!()
    }

    async fn cookies(&mut self) -> Result<Vec<Cookie>, Error> { unimplemented!() }

    async fn add_cookies(&mut self, cs: &[Cookie]) -> Result<(), Error> { unimplemented!() }

    async fn clear_cookies(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn grant_permission(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn clear_permission(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn set_geolocation(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn set_extra_http_headers(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn set_offline(&mut self, offline: bool) -> Result<(), Error> { unimplemented!() }

    async fn add_init_script(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn expose_binding(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn expose_function(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn route(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn unroute(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn expect_event(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn close(&mut self) -> Result<(), Error> { unimplemented!() }

    async fn storage_state(&mut self) -> Result<StorageState, Error> { unimplemented!() }

    async fn wait_for_event(&mut self) -> Result<StorageState, Error> { unimplemented!() }

    async fn expect_page(&mut self) -> Result<StorageState, Error> { unimplemented!() }
}
