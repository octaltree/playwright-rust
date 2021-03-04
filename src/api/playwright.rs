use crate::{
    imp::{self, core::*, playwright::*, prelude::*},
    BrowserType, Selectors
};
use std::{env, io, process::Command};

#[derive(Debug, thiserror::Error)]
pub enum PlaywrightError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error("Failed to intialize")]
    Initialization
}

pub struct Playwright {
    driver: Driver,
    _conn: Rc<Mutex<Connection>>,
    inner: Rweak<imp::playwright::Playwright>
}

impl Playwright {
    /// Installs playwright driver to "$CACHE_DIR/.ms-playwright/playwright-rust/driver"
    pub async fn initialize() -> Result<Playwright, PlaywrightError> {
        let driver = Driver::install()?;
        Self::with_driver(driver).await
    }

    /// Constructs from installed playwright driver
    pub async fn with_driver(driver: Driver) -> Result<Playwright, PlaywrightError> {
        let conn = driver.connect().await?;
        let p = Connection::wait_initial_object(Rc::downgrade(&conn)).await?;
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

    /// Launcher
    pub fn chromium(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.chromium.clone());
        BrowserType::new(inner)
    }

    /// Launcher
    pub fn firefox(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.firefox.clone());
        BrowserType::new(inner)
    }

    /// Launcher
    pub fn webkit(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.webkit.clone());
        BrowserType::new(inner)
    }

    pub fn driver(&mut self) -> &mut Driver { &mut self.driver }

    pub fn selectors(&self) -> Selectors {
        let inner = weak_and_then(&self.inner, |rc| rc.selectors.clone());
        Selectors::new(inner)
    }

    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner).unwrap().devices().to_vec()
    }
}
