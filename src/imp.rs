pub(crate) mod prelude {
    pub use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
    pub use serde_json::{map::Map, value::Value};
    pub use std::{
        cell::RefCell,
        collections::HashMap,
        convert::{TryFrom, TryInto},
        rc::{Rc, Weak},
        sync::Mutex
    };
    pub use strong::*;
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! find_object {
        ($conn:expr, $guid:expr, $t:ident) => {
            match $conn.get_object($guid) {
                Some(RemoteWeak::$t(x)) => Ok(x),
                _ => Err(ConnectionError::ObjectNotFound)
            }
        };
    }
}

pub(crate) mod connection;
pub(crate) mod driver;
pub(crate) mod message;
pub(crate) mod remote_object;
pub(crate) mod transport;

pub(crate) mod browser_type;
pub(crate) mod playwright;
pub(crate) mod selectors;

pub(crate) mod browser;
// pub(crate) mod browser_context;
// pub(crate) mod cdp_session;
// pub(crate) mod console_manager;
// pub(crate) mod dialog;
// pub(crate) mod download;
// pub(crate) mod frame;
// pub(crate) mod js_handle;
// pub(crate) mod network;
// pub(crate) mod page;

//_accessibility.py
//_api_structures.py
//_api_types.py
//_async_base.py
//_chromium_browser_context.py
//_connection.py
//_element_handle.py
//_event_context_manager.py
//_file_chooser.py
//_helper.py
//_impl_to_api_mapping.py
//_input.py
//_path_utils.py
//_sync_base.py
//_video.py
//_wait_helper.py
