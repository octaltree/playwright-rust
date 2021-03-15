# 🎭 [Playwright](https://playwright.dev) for Rust

[![crates.io](https://img.shields.io/crates/v/playwright)](https://crates.io/crates/strong) [![docs.rs](https://docs.rs/playwright/badge.svg)](https://docs.rs/playwright/) ![MIT](https://img.shields.io/crates/l/strong) ![](https://github.com/octaltree/playwright-rust/workflows/Test/badge.svg)

Playwright is a rust library to automate [Chromium](https://www.chromium.org/Home), [Firefox](https://www.mozilla.org/en-US/firefox/new/) and [WebKit](https://webkit.org/) built on top of Node.js library.

## Installation
```
[dependencies]
playwright = "0.0.2"
```

## Usage
```rust
use playwright::Playwright;

#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    env_logger::init();
    let mut playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let mut chromium = playwright.chromium();
    let mut browser = chromium.launcher().headless(true).launch().await?;
    let mut context = browser.context_builder().build().await?;
    let mut page = context.new_page().await?;
    page.goto_builder("https://example.com/").goto().await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    Ok(())
}
```

It's still under development and has limited functions. Please have a look at tests and [docs.rs](https://docs.rs/playwright/).
Welcome contributions.

## Async runtimes
* [tokio](https://crates.io/crates/tokio)
* [actix-rt](https://crates.io/crates/actix-rt)
* [async-std](https://crates.io/crates/async-std)
These runtimes have passed tests. You can disable tokio, the default feature, and then choose another.

## Incompatibility
Functions do not have default arguments in rust.
Functions with two or more optional arguments are now passed with the builder pattern.

## Playwright Driver
Playwright is designed as a server-client. All playwright client dependent on the driver: zip of core js library and Node.js.
Application uses this library will be bundled the driver into rust binary at build time. There is an overhead of unzipping on the first run.

## Browser automation in rust
- [atroche/rust-headless-chrome](https://github.com/atroche/rust-headless-chrome)
  * or [my fork](https://github.com/octaltree/rust-headless-chrome) supports set_cookies
- [saresend/selenium-rs](https://github.com/saresend/selenium-rs)
- [https://crates.io/crates/webdriver](https://crates.io/crates/webdriver)

## Other languages
- [microsoft/playwright](https://github.com/microsoft/playwright)
- [microsoft/playwright-python](https://github.com/microsoft/playwright-python)
- [microsoft/playwright-sharp](https://github.com/microsoft/playwright-sharp)
- [microsoft/playwright-java](https://github.com/microsoft/playwright-java)
- [mxschmitt/playwright-go](https://github.com/mxschmitt/playwright-go)
- [YusukeIwaki/playwright-ruby-client](https://github.com/YusukeIwaki/playwright-ruby-client)
