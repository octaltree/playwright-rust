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

pub fn default_driver_dest() -> PathBuf {
    let tmp: PathBuf = dirs::cache_dir().unwrap_or_else(env::temp_dir);
    let dir: PathBuf = tmp.join("ms-playwright/playwright-rust/driver");
    dir
}

pub struct Playwright {
    driver: Driver,
    _conn: Rc<Mutex<Connection>>,
    inner: Rweak<imp::playwright::Playwright>
}

impl Playwright {
    /// Installs playwright driver to "$CACHE_DIR/.ms-playwright/playwright-rust/driver"
    pub async fn initialize() -> Result<Playwright, PlaywrightError> {
        let dir = default_driver_dest();
        let driver = Driver::try_new(dir)?;
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
        BrowserType::new(upgrade(&self.inner).unwrap().chromium.clone())
    }

    /// Launcher
    pub fn firefox(&self) -> BrowserType {
        BrowserType::new(upgrade(&self.inner).unwrap().firefox.clone())
    }

    /// Launcher
    pub fn webkit(&self) -> BrowserType {
        BrowserType::new(upgrade(&self.inner).unwrap().webkit.clone())
    }

    pub fn driver(&mut self) -> &mut Driver { &mut self.driver }

    pub fn selectors(&self) -> Selectors {
        Selectors::new(upgrade(&self.inner).unwrap().selectors.clone())
    }

    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner).unwrap().devices().to_vec()
    }
}
