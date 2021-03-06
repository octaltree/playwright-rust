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

mod accessibility;
mod browser;
mod browser_context;
mod browser_type;
mod cdp_session;
mod console_message;
mod dialog;
mod download;
mod element_handle;
mod file_chooser;
mod frame;
mod input_device;
mod js_handle;
mod page;
mod request;
mod response;
mod route;
mod selectors;
mod video;
mod websocket;
mod worker;

pub use crate::imp::utils::*;

pub use accessibility::*;
pub use browser::*;
pub use browser_context::*;
pub use browser_type::*;
pub use cdp_session::*;
pub use console_message::*;
pub use dialog::*;
pub use download::*;
pub use element_handle::*;
pub use file_chooser::*;
pub use frame::*;
pub use input_device::*;
pub use js_handle::*;
pub use page::*;
pub use playwright::*;
pub use request::*;
pub use response::*;
pub use route::*;
pub use selectors::*;
pub use video::*;
pub use websocket::*;
pub use worker::*;

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
