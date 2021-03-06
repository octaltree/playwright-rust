use crate::{
    api::{browser_context::BrowserContext, page::Page},
    imp::{self, core::*, prelude::*},
    Error
};

pub struct Browser {
    inner: Weak<imp::browser::Browser>
}

impl Browser {
    pub(crate) fn new(inner: Weak<imp::browser::Browser>) -> Self { Self { inner } }

    pub fn contexts(&self) -> Result<Vec<BrowserContext>, Error> {
        Ok(upgrade(&self.inner)?
            .contexts()
            .iter()
            .cloned()
            .map(BrowserContext::new)
            .collect())
    }

    pub fn version(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.version().to_owned())
    }

    pub fn exists(&self) -> bool { self.inner.upgrade().is_some() }

    async fn new_context(&mut self) -> Result<BrowserContext, Error> { unimplemented!() }

    /// Shortcut of [`BrowserContext::new_page`] and [`Browser::new_context`].
    async fn new_page(&mut self) -> Result<Page, Error> { unimplemented!() }

    /// All browsers will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
    pub async fn close(&mut self) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.close().await
    }
}

// TODO: async drop
