use crate::{
    browser_context::BrowserContext,
    imp::{self, core::*, prelude::*},
    page::Page,
    Error
};

pub struct Browser {
    inner: Weak<imp::browser::Browser>
}

impl Browser {
    pub(crate) fn new(inner: Weak<imp::browser::Browser>) -> Self { Self { inner } }

    pub fn contexts(&self) -> Vec<BrowserContext> { unimplemented!() }

    pub fn version(&self) -> String { unimplemented!() }

    pub fn is_connntected(&self) -> bool { unimplemented!() }

    pub async fn new_context(&mut self) -> Result<BrowserContext, Error> { unimplemented!() }

    pub async fn new_page(&mut self) -> Result<Page, Error> { unimplemented!() }

    pub async fn close(&mut self) -> Result<(), Error> { unimplemented!() }
}
