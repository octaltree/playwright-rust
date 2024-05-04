use std::{
    env, fmt, fs,
    fs::File,
    path::{Path, PathBuf, MAIN_SEPARATOR}
};

const DRIVER_VERSION: &str = "1.13.0";

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let dest = out_dir.join("driver.zip");
    let platform = PlaywrightPlatform::default();
    fs::write(out_dir.join("platform"), platform.to_string()).unwrap();
    download(&url(platform), &dest);
    println!("cargo::rerun-if-changed=src/build.rs");
    println!("cargo::rustc-env=SEP={}", MAIN_SEPARATOR);
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
        let cache_is_file = || {
            maybe_metadata
                .as_ref()
                .map(fs::Metadata::is_file)
                .unwrap_or_default()
        };
        let cache_size = || {
            maybe_metadata
                .as_ref()
                .map(fs::Metadata::len)
                .unwrap_or_default()
        };
        if cache_is_file() && cache_size() > 10000000 {
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

fn size(p: &Path) -> u64 {
    let maybe_metadata = p.metadata().ok();
    let size = maybe_metadata
        .as_ref()
        .map(fs::Metadata::len)
        .unwrap_or_default();
    size
}

fn check_size(p: &Path) {
    assert!(size(p) > 10_000_000, "file size is smaller than the driver");
}

// No network access
#[cfg(feature = "only-for-docs-rs")]
fn download(_url: &str, dest: &Path) { File::create(dest).unwrap(); }

fn url(platform: PlaywrightPlatform) -> String {
    format!(
        "https://playwright.azureedge.net/builds/driver/playwright-{}-{}.zip",
        DRIVER_VERSION, platform
    )
}

#[derive(Clone, Copy)]
enum PlaywrightPlatform {
    Linux,
    Win32,
    Win32x64,
    Mac
}

impl fmt::Display for PlaywrightPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Win32 => write!(f, "win32"),
            Self::Win32x64 => write!(f, "win32_x64"),
            Self::Mac => write!(f, "mac")
        }
    }
}

impl Default for PlaywrightPlatform {
    fn default() -> Self {
        match env::var("CARGO_CFG_TARGET_OS").as_deref() {
            Ok("linux") => return PlaywrightPlatform::Linux,
            Ok("macos") => return PlaywrightPlatform::Mac,
            _ => ()
        };
        if env::var("CARGO_CFG_WINDOWS").is_ok() {
            if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").as_deref() == Ok("64") {
                PlaywrightPlatform::Win32x64
            } else {
                PlaywrightPlatform::Win32
            }
        } else if env::var("CARGO_CFG_UNIX").is_ok() {
            PlaywrightPlatform::Linux
        } else {
            panic!("Unsupported plaform");
        }
    }
}
