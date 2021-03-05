use crate::{
    imp::{self, core::*, prelude::*},
    utils::DeviceDescriptor
};
use std::{io, process::Command};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Connection(#[from] crate::imp::core::ConnectionError),
    #[error(transparent)]
    ConnectionArc(#[from] Arc<crate::imp::core::ConnectionError>),
    #[error("Failed to intialize")]
    Initialization,
    #[error(transparent)]
    Timeout(#[from] TimeoutError)
}

#[derive(Debug, thiserror::Error)]
#[error("")]
pub struct TimeoutError {}

pub struct Playwright {
    driver: Driver,
    _conn: Connection,
    inner: Weak<imp::playwright::Playwright>
}

impl Playwright {
    /// Installs playwright driver to "$CACHE_DIR/.ms-playwright/playwright-rust/driver"
    pub async fn initialize() -> Result<Playwright, Error> {
        let driver = Driver::install()?;
        Self::with_driver(driver).await
    }

    /// Constructs from installed playwright driver
    pub async fn with_driver(driver: Driver) -> Result<Playwright, Error> {
        let conn = Connection::run(&driver.executable())?;
        let p = imp::playwright::Playwright::wait_initial_object(&conn).await?;
        Ok(Self {
            driver,
            _conn: conn,
            inner: p
        })
    }

    /// Runs $ playwright install
    pub fn prepare(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install"])
            .status()?;
        Ok(())
    }

    ///// Launcher
    // pub fn chromium(&self) -> BrowserType {
    //    let inner = weak_and_then(&self.inner, |rc| rc.chromium.clone());
    //    BrowserType::new(inner)
    //}

    ///// Launcher
    // pub fn firefox(&self) -> BrowserType {
    //    let inner = weak_and_then(&self.inner, |rc| rc.firefox.clone());
    //    BrowserType::new(inner)
    //}

    ///// Launcher
    // pub fn webkit(&self) -> BrowserType {
    //    let inner = weak_and_then(&self.inner, |rc| rc.webkit.clone());
    //    BrowserType::new(inner)
    //}

    pub fn driver(&mut self) -> &mut Driver { &mut self.driver }

    // pub fn selectors(&self) -> Selectors {
    //    let inner = weak_and_then(&self.inner, |rc| rc.selectors.clone());
    //    Selectors::new(inner)
    //}

    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner).unwrap().devices().to_vec()
    }
}
