pub use crate::imp::playwright::DeviceDescriptor;
use crate::{
    api::{browser_type::BrowserType, selectors::Selectors},
    imp::{core::*, playwright::Playwright as Impl, prelude::*},
    Error
};
use std::{io, process::Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Entry point
pub struct Playwright {
    driver: Driver,
    _conn: Connection,
    inner: Weak<Impl>
}

fn run(driver: &Driver, args: &'static [&'static str]) -> io::Result<()> {
    let mut command = Command::new(driver.executable());
    let child = command.args(args);
    #[cfg(target_os = "windows")]
    child.creation_flags(0x08000000);
    let status = child.status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Exit with {}", status)
        ));
    }
    Ok(())
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
        let p = Impl::wait_initial_object(&conn).await?;
        Ok(Self {
            driver,
            _conn: conn,
            inner: p
        })
    }

    /// Runs $ playwright install
    pub fn prepare(&self) -> io::Result<()> { run(&self.driver, &["install"]) }

    /// Runs $ playwright install chromium
    pub fn install_chromium(&self) -> io::Result<()> { run(&self.driver, &["install", "chromium"]) }

    pub fn install_firefox(&self) -> io::Result<()> { run(&self.driver, &["install", "firefox"]) }

    pub fn install_webkit(&self) -> io::Result<()> { run(&self.driver, &["install", "webkit"]) }

    /// Launcher
    pub fn chromium(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.chromium());
        BrowserType::new(inner)
    }

    /// Launcher
    pub fn firefox(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.firefox());
        BrowserType::new(inner)
    }

    /// Launcher
    pub fn webkit(&self) -> BrowserType {
        let inner = weak_and_then(&self.inner, |rc| rc.webkit());
        BrowserType::new(inner)
    }

    pub fn driver(&mut self) -> &mut Driver { &mut self.driver }

    pub fn selectors(&self) -> Selectors {
        let inner = weak_and_then(&self.inner, |rc| rc.selectors());
        Selectors::new(inner)
    }

    /// Returns a dictionary of devices to be used with [`method: Browser.newContext`] or [`method: Browser.newPage`].
    ///
    /// ```js
    /// const { webkit, devices } = require('playwright');
    /// const iPhone = devices['iPhone 6'];
    ///
    /// (async () => {
    ///  const browser = await webkit.launch();
    ///  const context = await browser.newContext({
    ///    ...iPhone
    ///  });
    ///  const page = await context.newPage();
    ///  await page.goto('http://example.com');
    ///  // other actions...
    ///  await browser.close();
    /// })();
    /// ```
    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner)
            .map(|x| x.devices().to_vec())
            .unwrap_or_default()
    }

    pub fn device(&self, name: &str) -> Option<DeviceDescriptor> {
        let inner = self.inner.upgrade()?;
        let device = inner.device(name)?;
        Some(device.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::runtime_test!(initialize, {
        let _ = Playwright::initialize().await.unwrap();
    });
}
