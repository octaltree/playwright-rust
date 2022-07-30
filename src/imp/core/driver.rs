use crate::imp::prelude::*;
use std::{env, fmt, fs, io, str::FromStr};
use zip::{result::ZipError, ZipArchive};

#[derive(Debug, Clone, PartialEq)]
pub struct Driver {
    path: PathBuf
}

impl Driver {
    const ZIP: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), env!("SEP"), "driver.zip"));
    const PLATFORM: &'static str = include_str!(concat!(env!("OUT_DIR"), env!("SEP"), "platform"));

    pub fn install() -> io::Result<Self> {
        let this = Self::new(Self::default_dest());
        if !this.path.is_dir() {
            this.prepare()?;
        }
        Ok(this)
    }

    /// Without prepare
    pub fn new<P: Into<PathBuf>>(path: P) -> Self { Self { path: path.into() } }
    ///
    pub fn prepare(&self) -> Result<(), ZipError> {
        fs::create_dir_all(&self.path)?;
        let mut a = ZipArchive::new(io::Cursor::new(Self::ZIP))?;
        a.extract(&self.path)
    }

    pub fn default_dest() -> PathBuf {
        let base: PathBuf = dirs::cache_dir().unwrap_or_else(env::temp_dir);
        let dir: PathBuf = [
            base.as_os_str(),
            "ms-playwright".as_ref(),
            "playwright-rust".as_ref(),
            "driver".as_ref()
        ]
        .iter()
        .collect();
        dir
    }

    pub fn platform(&self) -> Platform { Platform::from_str(Self::PLATFORM).unwrap() }

    pub fn executable(&self) -> PathBuf {
        match self.platform() {
            Platform::Linux => self.path.join("playwright.sh"),
            Platform::LinuxArm64 => self.path.join("playwright.sh"),
            Platform::Mac => self.path.join("playwright.sh"),
            Platform::MacArm64 => self.path.join("playwright.sh"),
            Platform::Win32X64 => self.path.join("playwright.cmd")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Mac,
    MacArm64,
    Linux,
    LinuxArm64,
    Win32X64
}

const LABEL: &[(Platform, &str)] = &[
    (Platform::Mac, "mac"),
    (Platform::MacArm64, "mac-arm64"),
    (Platform::Linux, "linux"),
    (Platform::LinuxArm64, "linux-arm64"),
    (Platform::Win32X64, "win32_x64")
];

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hit = LABEL
            .into_iter()
            .find(|(a, _)| a == self)
            .map(|(_, s)| s)
            .unwrap();
        write!(f, "{}", hit)
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hit = LABEL
            .into_iter()
            .find(|&(_, b)| *b == s)
            .map(|&(a, _)| a)
            .ok_or(())?;
        Ok(hit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install() { let _driver = Driver::install().unwrap(); }
}
