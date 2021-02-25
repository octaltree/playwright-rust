use crate::imp::driver::Driver;
use std::{io, path::Path};

pub struct Playwright<'a> {
    driver: Driver<'a>
}

impl<'a> Playwright<'a> {
    fn initialize(path: &'a Path) -> io::Result<Self> {
        let driver = Driver::try_new(&path)?;
        Ok(Self { driver })
    }
}

impl<'a> Drop for Playwright<'a> {
    fn drop(&mut self) {}
}
