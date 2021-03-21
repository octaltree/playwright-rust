pub use crate::imp::browser_context::EventType;
use crate::{
    api::{Browser, Page},
    imp::{
        browser_context::{BrowserContext as Impl, Evt},
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

    ///// Returns the browser instance of the context. If it was launched as a persistent context None gets returned.
    pub fn browser(&self) -> Result<Option<Browser>, Error> {
        Ok(upgrade(&self.inner)?.browser().map(Browser::new))
    }

    pub async fn new_page(&self) -> Result<Page, Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        Ok(Page::new(inner.new_page().await?))
    }

    pub async fn set_default_navigation_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?
            .set_default_navigation_timeout(timeout)
            .await
    }

    pub async fn set_default_timeout(&self, timeout: u32) -> ArcResult<()> {
        upgrade(&self.inner)?.set_default_timeout(timeout).await
    }

    pub async fn cookies(&self, urls: &[String]) -> ArcResult<Vec<Cookie>> {
        upgrade(&self.inner)?.cookies(urls).await
    }

    pub async fn add_cookies(&self, cookies: &[Cookie]) -> ArcResult<()> {
        upgrade(&self.inner)?.add_cookies(cookies).await
    }

    pub async fn clear_cookies(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_cookies().await
    }

    pub async fn grant_permission(
        &self,
        permissions: &[String],
        origin: Option<&str>
    ) -> ArcResult<()> {
        upgrade(&self.inner)?
            .grant_permission(permissions, origin)
            .await
    }

    pub async fn clear_permissions(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.clear_permissions().await
    }

    pub async fn set_geolocation(&self, geolocation: Option<&Geolocation>) -> ArcResult<()> {
        upgrade(&self.inner)?.set_geolocation(geolocation).await
    }

    pub async fn set_offline(&self, offline: bool) -> ArcResult<()> {
        upgrade(&self.inner)?.set_offline(offline).await
    }

    pub async fn add_init_script(&self, script: &str) -> ArcResult<()> {
        upgrade(&self.inner)?.add_init_script(script).await
    }

    pub async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        upgrade(&self.inner)?.set_extra_http_headers(headers).await
    }

    // async fn expose_binding(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn expose_function(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn route(&mut self) -> Result<(), Error> { unimplemented!() }

    // async fn unroute(&mut self) -> Result<(), Error> { unimplemented!() }

    pub async fn expect_event(&self, evt: EventType) -> Result<Event, Error> {
        upgrade(&self.inner)?
            .expect_event(evt)
            .await
            .map(|e| match e {
                Evt::Close => Event::Close,
                Evt::Page(w) => Event::Page(Page::new(w))
            })
    }

    // pub fn subscribe_event(&self) -> Result<broadcast::Receiver<Event>, Error> {
    //    Ok(upgrade(&self.inner)?.subscribe_event())
    //}

    pub async fn storage_state(&self) -> ArcResult<StorageState> {
        upgrade(&self.inner)?.storage_state().await
    }

    /// All temporary browsers will be closed when the connection is terminated, but
    /// this struct has no Drop. it needs to be called explicitly to close it at any given time.
    pub async fn close(&self) -> ArcResult<()> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close().await
    }
}

pub enum Event {
    Close,
    Page(Page)
}
