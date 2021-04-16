pub use crate::imp::playwright::DeviceDescriptor;
use crate::{
    api::{browser_type::BrowserType, selectors::Selectors},
    imp::{core::*, playwright::Playwright as Impl, prelude::*},
    Error
};
use std::{io, process::Command};

/// Entry point
pub struct Playwright {
    driver: Driver,
    _conn: Connection,
    inner: Weak<Impl>
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
    pub fn prepare(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install"])
            .status()?;
        Ok(())
    }

    /// Runs $ playwright install chromium
    pub fn install_chromium(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install", "chromium"])
            .status()?;
        Ok(())
    }

    pub fn install_firefox(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install", "firefox"])
            .status()?;
        Ok(())
    }

    pub fn install_webkit(&self) -> io::Result<()> {
        Command::new(self.driver.executable())
            .args(&["install", "webkit"])
            .status()?;
        Ok(())
    }

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
    ///
    /// ```python async
    /// import asyncio
    /// from playwright.async_api import async_playwright
    ///
    /// async def run(playwright):
    ///    webkit = playwright.webkit
    ///    iphone = playwright.devices["iPhone 6"]
    ///    browser = await webkit.launch()
    ///    context = await browser.new_context(**iphone)
    ///    page = await context.new_page()
    ///    await page.goto("http://example.com")
    ///    # other actions...
    ///    await browser.close()
    ///
    /// async def main():
    ///    async with async_playwright() as playwright:
    ///        await run(playwright)
    /// asyncio.run(main())
    /// ```
    ///
    /// ```python sync
    /// from playwright.sync_api import sync_playwright
    ///
    /// def run(playwright):
    ///    webkit = playwright.webkit
    ///    iphone = playwright.devices["iPhone 6"]
    ///    browser = webkit.launch()
    ///    context = browser.new_context(**iphone)
    ///    page = context.new_page()
    ///    page.goto("http://example.com")
    ///    # other actions...
    ///    browser.close()
    ///
    /// with sync_playwright() as playwright:
    ///    run(playwright)
    /// ```
    pub fn devices(&self) -> Vec<DeviceDescriptor> {
        upgrade(&self.inner)
            .map(|x| x.devices().to_vec())
            .unwrap_or_default()
    }
}
