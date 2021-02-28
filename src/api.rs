use crate::imp::{
    self,
    connection::{Connection, ConnectionError},
    driver::Driver
};
use std::{io, path::Path, sync::Arc};
use thiserror::Error;

pub struct Playwright<'a> {
    driver: Driver<'a>,
    conn: Connection,
    entry_point: Arc<imp::playwright::Playwright>
}

#[derive(Debug, Error)]
pub enum PlaywrightError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error("Failed to intialize")]
    Initialization
}

impl<'a> Playwright<'a> {
    async fn initialize(path: &'a Path) -> Result<Playwright<'a>, PlaywrightError> {
        let driver = Driver::try_new(&path)?;
        let mut conn = driver.run().await?;
        let p = conn.wait_initial_object().await?;
        Ok(Self {
            driver,
            conn,
            entry_point: p
        })
    }

    // fn chromium
    // fn firefox
    // fn webkit
    // fn selectors
}

impl<'a> Drop for Playwright<'a> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    crate::runtime_test!(initialize, {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let _ = Playwright::initialize(&tmp).await.unwrap();
    });
}
