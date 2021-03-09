# 🎭 [Playwright](https://playwright.dev) for Rust

Playwright is a rust library to automate Chromium, Firefox and WebKit built on top of Node.js library.

## Installation
```
[dependencies]
playwright = "0.1.0"
```

## Usage
WIP


## Playwright Driver
Playwright is designed as a server-client. All playwright client dependent on driver: zip of core js library and node.
Application uses this library will be bundled the driver into rust binary at build time. There is an overhead of unzipping on the first run.

## Incompatibility
Functions do not have default arguments in rust.
Functions with two or more optional arguments are now passed with the builder pattern.

## Browser automation in rust
- [atroche/rust-headless-chrome](https://github.com/atroche/rust-headless-chrome)
  * or my fork suuports set_cookies [octaltree/rust-headless-chrome](https://github.com/octaltree/rust-headless-chrome)
- [saresend/selenium-rs](https://github.com/saresend/selenium-rs)
- [https://crates.io/crates/webdrive](https://crates.io/crates/webdrive)

## Other languages
- [microsoft/playwright](https://github.com/microsoft/playwright)
- [microsoft/playwright-python](https://github.com/microsoft/playwright-python)
- [microsoft/playwright-sharp](https://github.com/microsoft/playwright-sharp)
- [microsoft/playwright-java](https://github.com/microsoft/playwright-java)
- [mxschmitt/playwright-go](https://github.com/mxschmitt/playwright-go)
- [YusukeIwaki/playwright-ruby-client](https://github.com/YusukeIwaki/playwright-ruby-client)
