use crate::imp::{core::*, prelude::*};
use std::{
    fs, io,
    path::{Path, PathBuf}
};
use zip::{result::ZipError, ZipArchive};

pub(crate) struct Driver<'a> {
    path: &'a Path
}

impl<'a> Driver<'a> {
    const ZIP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/driver.zip"));
    const PLATFORM: &'static str = include_str!(concat!(env!("OUT_DIR"), "/platform"));

    pub(crate) fn try_new(path: &'a Path) -> io::Result<Self> {
        let this = Self { path };
        this.prepare()?;
        Ok(this)
    }

    pub(crate) async fn run(&self) -> io::Result<Rc<Mutex<Connection>>> {
        Connection::try_new(&self.executable()).await
    }

    fn prepare(&self) -> Result<(), ZipError> {
        if self.path.is_dir() {
            return Ok(());
        }
        fs::create_dir_all(self.path)?;
        let mut a = ZipArchive::new(io::Cursor::new(Self::ZIP))?;
        a.extract(self.path)
    }

    fn platform(&self) -> Platform {
        match Self::PLATFORM {
            "linux" => Platform::Linux,
            "mac" => Platform::Mac,
            "win32" => Platform::Win32,
            "win32_x64" => Platform::Win32x64,
            _ => unreachable!()
        }
    }

    pub(crate) fn executable(&self) -> PathBuf {
        match self.platform() {
            Platform::Linux => self.path.join("playwright.sh"),
            Platform::Mac => self.path.join("playwright.sh"),
            Platform::Win32 => self.path.join("playwright.cmd"),
            Platform::Win32x64 => self.path.join("playwright.cmd")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Platform {
    Linux,
    Win32,
    Win32x64,
    Mac
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn write() {
        let tmp = env::temp_dir();
        let tmp = tmp.join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        assert_eq!(driver.executable().parent().unwrap(), &tmp);
    }
}
