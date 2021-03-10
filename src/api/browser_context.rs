use crate::{
    api::{Browser, Page},
    imp::{
        browser_context::BrowserContext as Impl,
        core::*,
        prelude::*,
        utils::{Cookie, Geolocation, StorageState}
    },
    Error
};

pub struct BrowserContext {
    inner: Weak<Impl>
}

impl BrowserContext {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn pages(&self) -> Result<Vec<Page>, Error> {
        Ok(upgrade(&self.inner)?
            .pages()
            .iter()
            .cloned()
            .map(Page::new)
            .collect())
    }

    pub fn browser(&self) -> Result<Option<Browser>, Error> {
        Ok(upgrade(&self.inner)?.browser().map(Browser::new))
    }

    pub async fn new_page(&mut self) -> Result<Page, Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        Ok(Page::new(inner.new_page().await?))
    }

    ///// Returns the browser instance of the context. If it was launched as a persistent context null gets returned.
    // fn browser(&self) -> Option<Browser> { unimplemented!() }

    // async fn set_default_navigation_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
    //    unimplemented!()
    //}

    // async fn set_default_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
    //    unimplemented!()
    //}

    pub async fn cookies(&mut self, urls: &[String]) -> ArcResult<Vec<Cookie>> {
        upgrade(&self.inner)?.cookies(urls).await
    }

    pub async fn add_cookies(&mut self, cookies: &[Cookie]) -> ArcResult<()> {
        upgrade(&self.inner)?.add_cookies(cookies).await
    }

    pub async fn clear_cookies(&mut self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_cookies().await
    }

    pub async fn grant_permission(
        &mut self,
        permissions: &[String],
        origin: Option<&str>
    ) -> ArcResult<()> {
        upgrade(&self.inner)?
            .grant_permission(permissions, origin)
            .await
    }

    pub async fn clear_permissions(&mut self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_permissions().await
    }

    pub async fn set_geolocation(&mut self, geolocation: Option<&Geolocation>) -> ArcResult<()> {
        upgrade(&self.inner)?.set_geolocation(geolocation).await
    }

    pub async fn set_offline(&mut self, offline: bool) -> ArcResult<()> {
        upgrade(&self.inner)?.set_offline(offline).await
    }

    pub async fn add_init_script(&mut self, script: &str) -> ArcResult<()> {
        upgrade(&self.inner)?.add_init_script(script).await
    }

    pub async fn set_extra_http_headers<T>(&mut self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        upgrade(&self.inner)?.set_extra_http_headers(headers).await
    }

    // async fn expose_binding(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn expose_function(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn route(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn unroute(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn expect_event(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn wait_for_event(&mut self) -> Result<StorageState, Error> { unimplemented!() }

    // async fn expect_page(&mut self) -> Result<StorageState, Error> { unimplemented!() }

    pub async fn storage_state(&mut self) -> ArcResult<StorageState> {
        upgrade(&self.inner)?.storage_state().await
    }

    /// All temporary browsers will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
    pub async fn close(&mut self) -> ArcResult<()> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close().await
    }
}
