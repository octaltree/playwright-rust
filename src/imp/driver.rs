use std::{fs, io, io::prelude::*, path::Path};
use zip::{result::ZipError, ZipArchive};

pub struct Driver<'a> {
    path: &'a Path
}

impl<'a> Driver<'a> {
    const ZIP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/driver.zip"));

    pub fn try_new(path: &'a Path) -> io::Result<Self> {
        let this = Self { path };
        this.prepare()?;
        Ok(this)
    }

    fn prepare(&self) -> Result<(), ZipError> {
        if self.path.is_dir() {
            return Ok(());
        }
        fs::create_dir_all(self.path)?;
        let mut a = ZipArchive::new(io::Cursor::new(Self::ZIP))?;
        a.extract(self.path)
    }

    pub fn executable(&self) -> &Path { self.path }
}

#[derive(Debug, Clone, Copy)]
enum Platform {
    Linux,
    Win32,
    Win32x64,
    Mac
}

impl Platform {
    const SERIALIZED: &'static str = include_str!(concat!(env!("OUT_DIR"), "/platform"));
}

impl Default for Platform {
    fn default() -> Self {
        match Self::SERIALIZED {
            "linux" => Self::Linux,
            "mac" => Self::Mac,
            "win32" => Self::Win32,
            "win32_x64" => Self::Win32x64,
            _ => unreachable!()
        }
    }
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
        assert_eq!(driver.executable(), &tmp);
        fs::remove_dir_all(&tmp).unwrap();
    }
}
