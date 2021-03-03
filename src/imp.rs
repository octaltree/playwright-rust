pub(crate) mod prelude {
    pub use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{
        map::Map,
        value::{to_value, Value}
    };
    pub use std::{
        cell::RefCell,
        collections::HashMap,
        convert::{TryFrom, TryInto},
        rc::{Rc, Weak as Rweak},
        sync::{Arc, Mutex, Weak as Aweak},
        task::Waker
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

    #[macro_export]
    macro_rules! send_message {
        ($r: expr, $method:expr, $args: expr) => {{
            let r = $r.channel().create_request($method).set_args($args)?;
            let fut = $r.channel().send_message(r).await?;
            let res = fut.await?;
            let res = res.map_err(ConnectionError::ErrorResponded)?;
            res
        }};
    }
}

pub(crate) mod core {
    mod connection;
    mod driver;
    mod message;
    mod remote_object;
    mod transport;
    pub(crate) use connection::*;
    pub(crate) use driver::*;
    pub(crate) use message::*;
    pub(crate) use remote_object::*;
    pub(crate) use transport::*;
}

pub(crate) mod browser_type;
pub(crate) mod playwright;
pub(crate) mod selectors;

pub(crate) mod browser;
pub(crate) mod browser_context;
pub(crate) mod cdp_session;
pub(crate) mod console_manager;
pub(crate) mod dialog;
pub(crate) mod download;
pub(crate) mod frame;
pub(crate) mod js_handle;
pub(crate) mod network;
pub(crate) mod page;

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
