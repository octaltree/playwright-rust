use crate::imp::{connection::Connection, driver::Driver};
use std::{io, path::Path};

pub struct Playwright<'a> {
    driver: Driver<'a>,
    conn: Connection
}

impl<'a> Playwright<'a> {
    async fn initialize(path: &'a Path) -> io::Result<Playwright<'a>> {
        let driver = Driver::try_new(&path)?;
        let mut conn = driver.run().await?;
        Ok(Self { driver, conn })
    }

    // fn chromium
    // fn firefox
    // fn webkit
    // fn selectors
}
