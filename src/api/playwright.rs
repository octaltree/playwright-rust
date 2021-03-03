use crate::imp::{self, core::*, prelude::*};
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
    let tmp: PathBuf = dirs::cache_dir().unwrap_or_else(|| env::temp_dir());
    let dir: PathBuf = tmp.join("ms-playwright/playwright-rust/driver");
    dir
}

pub struct Playwright {
    driver: Driver,
    conn: Rc<Mutex<Connection>>,
    inner: Rweak<imp::playwright::Playwright>
}

impl Playwright {
    pub async fn with_driver(driver: Driver) -> Result<Playwright, PlaywrightError> {
        let conn = driver.connect().await?;
        let p = Connection::wait_initial_object(Rc::downgrade(&conn)).await?;
        Ok(Self {
            driver,
            conn,
            inner: p
        })
    }

    pub async fn initialize() -> Result<Playwright, PlaywrightError> {
        let dir = default_driver_dest();
        let driver = Driver::try_new(dir)?;
        Self::with_driver(driver).await
    }

    pub fn prepare(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install"])
            .status()?;
        Ok(())
    }

    // fn devices(&self) -> HashMap<String, String> { unimplemented!() }
    // fn selectors(&self) -> &Selectors { unimplemented!() }
    // fn chromium(&self) -> &BrowserType { unimplemented!() }
    // fn firefox(&self) -> &BrowserType { unimplemented!() }
    // fn webkit(&self) -> &BrowserType { unimplemented!() }

    pub fn driver(&mut self) -> &mut Driver { &mut self.driver }
}
