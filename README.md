# 🎭 [Playwright](https://playwright.dev) for Rust

[![crates.io](https://img.shields.io/crates/v/playwright)](https://crates.io/crates/playwright)
[![docs.rs](https://docs.rs/playwright/badge.svg)](https://docs.rs/playwright/)
![MIT OR Apache-2.0](https://img.shields.io/crates/l/playwright)
[![CI](https://github.com/octaltree/playwright-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/octaltree/playwright-rust/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/octaltree/playwright-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/octaltree/playwright-rust)

Playwright is a rust library to automate [Chromium](https://www.chromium.org/Home), [Firefox](https://www.mozilla.org/en-US/firefox/new/) and [WebKit](https://webkit.org/) built on top of Node.js library.

## Installation
```
[dependencies]
playwright = "0.0.20"
```

## Usage
```rust
use playwright::Playwright;

#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://example.com/").goto().await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    Ok(())
}
```

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

### NOTICE
```
playwright-rust redistributes Playwright licensed under the Apache 2.0.
Playwright has NOTICE:
"""
Playwright
Copyright (c) Microsoft Corporation

This software contains code derived from the Puppeteer project (https://github.com/puppeteer/puppeteer),
available under the Apache 2.0 license (https://github.com/puppeteer/puppeteer/blob/master/LICENSE).
"""
```

## Browser automation in rust
- [atroche/rust-headless-chrome](https://github.com/atroche/rust-headless-chrome)
- [saresend/selenium-rs](https://github.com/saresend/selenium-rs)
- [https://crates.io/crates/webdriver](https://crates.io/crates/webdriver)
- [mattsse/chromiumoxide](https://github.com/mattsse/chromiumoxide)

## Other languages
- [microsoft/playwright](https://github.com/microsoft/playwright)
    * [Documentation](https://playwright.dev/docs/intro/)
    * [API Reference](https://playwright.dev/docs/api/class-playwright/)
- [microsoft/playwright-python](https://github.com/microsoft/playwright-python)
- [microsoft/playwright-sharp](https://github.com/microsoft/playwright-sharp)
- [microsoft/playwright-java](https://github.com/microsoft/playwright-java)
- [mxschmitt/playwright-go](https://github.com/mxschmitt/playwright-go)
- [YusukeIwaki/playwright-ruby-client](https://github.com/YusukeIwaki/playwright-ruby-client)
- [teodesian/playwright-perl](https://github.com/teodesian/playwright-perl)
- [luka-dev/playwright-php](https://github.com/luka-dev/playwright-php)
- [naqvis/playwright-cr](https://github.com/naqvis/playwright-cr)
- [geometerio/playwright-elixir](https://github.com/geometerio/playwright-elixir)
