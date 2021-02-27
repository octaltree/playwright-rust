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
        conn.receive_initializer_message().await;
        Ok(Self { driver, conn })
    }
}

impl<'a> Drop for Playwright<'a> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    //#[crate::test]
    // async fn initialize() {
    //    // let tmp = env::temp_dir().join("playwright-rust-test/driver");
    //    // let _ = Playwright::initialize(&tmp).await.unwrap();
    //}
}
