#[doc(hidden)]
#[macro_export]
macro_rules! optional_setter {
    ($($field:ident, $t: ty);*) => {
        $(
            paste::paste! {
                pub fn [<$field>](mut self, x:$t ) -> Self {
                    self.args.$field = Some(x);
                    self
                }
            }
        )*
        $(
            paste::paste! {
                pub fn [<clear_$field>](mut self) -> Self {
                    self.args.$field = None;
                    self
                }
            }
        )*
    };
}

pub mod playwright;

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

pub use crate::imp::utils::*;

pub use accessibility::Accessibility;
pub use browser::Browser;
pub use browser_context::BrowserContext;
pub use browser_type::BrowserType;
pub use cdp_session::CdpSession;
pub use console_message::ConsoleMessage;
pub use dialog::Dialog;
pub use download::Download;
pub use element_handle::ElementHandle;
pub use file_chooser::FileChooser;
pub use frame::Frame;
pub use input_device::{Keyboard, Mouse, TouchScreen};
pub use js_handle::JsHandle;
pub use page::Page;
pub use playwright::Playwright;
pub use request::Request;
pub use response::Response;
pub use route::Route;
pub use selectors::Selectors;
pub use video::Video;
pub use websocket::WebSocket;
pub use worker::Worker;

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
