mod playwright;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Connection(#[from] crate::imp::core::ConnectionError),
    #[error("Failed to intialize")]
    Initialization,
    #[error(transparent)]
    Timeout(#[from] TimeoutError)
}

pub mod accessibility;
pub mod browser;
pub mod browser_context;
pub mod browser_type;
pub mod cdp_session;
pub mod console_message;
pub mod dialog;
pub mod download;
pub mod element_handle;
pub mod file_chooser;
pub mod frame;
pub mod input_device;
pub mod js_handle;
pub mod page;
pub mod request;
pub mod response;
pub mod route;
pub mod selectors;
pub mod video;
pub mod websocket;
pub mod worker;

pub use crate::imp::core::Driver;
pub use playwright::*;
// pub use accessibility::Accessibility;
// pub use browser::Browser;
// pub use browser_context::BrowserContext;
// pub use browser_type::BrowserType;
// pub use cdp_session::CdpSession;
// pub use console_message::ConsoleMessage;
// pub use dialog::Dialog;
// pub use download::Download;
// pub use element_handle::ElementHandle;
// pub use file_chooser::FileChooser;
// pub use frame::Frame;
// pub use input_device::{Keyboard, Mouse};
// pub use js_handle::JsHandle;
// pub use page::Page;
// pub use playwright::{Playwright, PlaywrightError, TimeoutError};
// pub use request::Request;
// pub use response::Response;
// pub use route::Route;
// pub use selectors::Selectors;
// pub use video::Video;
// pub use websocket::WebSocket;
// pub use worker::Worker;

// BindingCall
// ChromiumBrowserContext
// Cookie
// FilePayload
// FloatRect
// Geolocation
// HttpCredentials
// PdfMargins
// Position
// ProxySettings
// ResourceTiming
// SourceLocation
// StorageState
// ViewportSize

// Android
// AndroidDevice
// AndroidInput
// AndroidSocket
// AndroidWebView
// BrowserServer
// ChromiumBrowser
// ChromiumBrowserContext
// ChromiumCoverage
// Electron
// ElectronApplication
// FirefoxBrowser
// Logger
// WebKitBrowser
// WebSocketFrame
