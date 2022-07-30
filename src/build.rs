use std::{
    env, fmt, fs,
    fs::File,
    path::{Path, PathBuf, MAIN_SEPARATOR}
};

const DRIVER_VERSION: &str = "1.25.0-alpha-jul-26-2022";

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let dest = out_dir.join("driver.zip");
    let platform = Platform::default();
    fs::write(out_dir.join("platform"), platform.to_string()).unwrap();
    download(&url(platform), &dest);
    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rustc-env=SEP={}", MAIN_SEPARATOR);
}

#[cfg(all(not(feature = "only-for-docs-rs"), not(unix)))]
fn download(url: &str, dest: &Path) {
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut dest = File::create(dest).unwrap();
    resp.copy_to(&mut dest).unwrap();
}

#[cfg(all(not(feature = "only-for-docs-rs"), unix))]
fn download(url: &str, dest: &Path) {
    let cache_dir: &Path = "/tmp/build-playwright-rust".as_ref();
    let cached = cache_dir.join("driver.zip");
    if cfg!(debug_assertions) {
        let maybe_metadata = cached.metadata().ok();
        let cache_is_file = maybe_metadata
            .as_ref()
            .map(fs::Metadata::is_file)
            .unwrap_or_default();
        let cache_size = maybe_metadata
            .as_ref()
            .map(fs::Metadata::len)
            .unwrap_or_default();
        if cache_is_file && cache_size > 10000000 {
            fs::copy(cached, dest).unwrap();
            check_size(dest);
            return;
        }
    }
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut dest_file = File::create(dest).unwrap();
    resp.copy_to(&mut dest_file).unwrap();
    if cfg!(debug_assertions) {
        fs::create_dir_all(cache_dir).unwrap();
        fs::copy(dest, cached).unwrap();
    }
    check_size(dest);
}

fn check_size(p: &Path) {
    assert!(size(p) > 10_000_000, "file size is smaller than the driver");
}

fn size(p: &Path) -> u64 {
    let maybe_metadata = p.metadata().ok();
    let size = maybe_metadata
        .as_ref()
        .map(fs::Metadata::len)
        .unwrap_or_default();
    size
}

// No network access
#[cfg(feature = "only-for-docs-rs")]
fn download(_url: &str, dest: &Path) { File::create(dest).unwrap(); }

fn url(platform: Platform) -> String {
    let next = (DRIVER_VERSION.contains("-next")
        || DRIVER_VERSION.contains("-alpha")
        || DRIVER_VERSION.contains("-beta"))
    .then(|| "next/")
    .unwrap_or_default();
    format!(
        "https://playwright.azureedge.net/builds/driver/{}playwright-{}-{}.zip",
        next, DRIVER_VERSION, platform
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Platform {
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

impl std::str::FromStr for Platform {
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

impl Default for Platform {
    fn default() -> Self {
        if env::var("CARGO_CFG_WINDOWS").is_ok() {
            return Self::Win32X64;
        }
        match env::var("CARGO_CFG_TARGET_OS").as_deref() {
            Ok("linux") => {
                if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "aarch64" {
                    Self::LinuxArm64
                } else {
                    Self::Linux
                }
            }
            Ok("macos") => {
                if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "aarch64" {
                    Self::MacArm64
                } else {
                    Self::Mac
                }
            }
            _ => panic!("Unsupported plaform")
        }
    }
}
