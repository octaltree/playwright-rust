use crate::imp::{
    self,
    connection::{Connection, ConnectionError},
    driver::Driver,
    prelude::*
};
use std::{io, path::Path};
use thiserror::Error;

pub struct Playwright<'a> {
    driver: Driver<'a>,
    conn: Rc<RefCell<Connection>>,
    inner: Weak<imp::playwright::Playwright>
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
        let conn = driver.run().await?;
        let p = conn.borrow_mut().wait_initial_object().await?;
        Ok(Self {
            driver,
            conn,
            inner: p
        })
    }

    // fn devices(&self) -> HashMap<String, String> { unimplemented!() }
    // fn selectors(&self) -> &Selectors { unimplemented!() }
    // fn chromium(&self) -> &BrowserType { unimplemented!() }
    // fn firefox(&self) -> &BrowserType { unimplemented!() }
    // fn webkit(&self) -> &BrowserType { unimplemented!() }
}

impl<'a> Drop for Playwright<'a> {
    fn drop(&mut self) {}
}

// struct Request{}

// impl Request{
//    fn url(&self) -> &str{unimplemented!()}
//    fn resource_type(&self) -> &str{unimplemented!()}
//    fn method(&self) -> &str{unimplemented!()}
//    fn post_data(&self) -> Option<&str>{unimplemented!()}
//}

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
