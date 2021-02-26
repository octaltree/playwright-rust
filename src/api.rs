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

    #[cfg(feature = "runtime-tokio")]
    #[tokio::test]
    async fn tokio_initialize() {
        let _ = Playwright::initialize("/tmp/playwright-rust-test/driver".as_ref())
            .await
            .unwrap();
    }
}
