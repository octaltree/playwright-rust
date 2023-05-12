pub(crate) mod impl_future {
    pub use std::{future::Future, pin::Pin, task};
}
pub(crate) mod prelude {
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use serde_json::{
        map::Map,
        value::{to_value, Value}
    };
    pub use parking_lot::{
        Mutex, MutexGuard
    };
    pub use std::{
        collections::HashMap,
        convert::{TryFrom, TryInto},
        future::Future,
        path::{Path, PathBuf},
        pin::Pin,
        sync::{
            Arc, Weak,
            // Mutex, MutexGuard
        },
        task::{Poll, Waker},
        time::Duration
    };
    pub use strong::*;
    pub type Wm<T> = Weak<Mutex<T>>;
    pub type Am<T> = Arc<Mutex<T>>;


    #[cfg(feature = "rt-async-std")]
    #[derive(Debug, thiserror::Error)]
    pub enum JoinError {}
    #[cfg(feature = "rt-async-std")]
    pub use async_std::{task::sleep, task::spawn};
    #[cfg(feature = "rt-tokio")]
    pub use tokio::{task::spawn, task::JoinError, time::sleep};
    #[cfg(feature = "rt-actix")]
    pub use tokio::{task::spawn, task::JoinError, time::sleep};

    pub(crate) trait RemoveOne<T> {
        fn remove_one<F>(&mut self, f: F)
        where
            F: Fn(&T) -> bool;
    }

    impl<T> RemoveOne<T> for Vec<T> {
        fn remove_one<F>(&mut self, f: F)
        where
            F: Fn(&T) -> bool
        {
            let index = match self.iter().position(f) {
                Some(i) => i,
                None => return
            };
            self.remove(index);
        }
    }
}

#[macro_use]
mod macros {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! get_object {
        ($c:expr, $guid:expr, $t:ident) => {
            match $c.find_object($guid) {
                Some(RemoteWeak::$t(x)) => Ok(x),
                _ => Err(Error::ObjectNotFound)
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! send_message {
        ($r: expr, $method:literal, $args: expr) => {{
            let m: Str<Method> = $method.to_owned().try_into().unwrap();
            let r = $r.channel().create_request(m).set_args($args)?;
            let fut = $r.channel().send_message(r).await?;
            let res = fut.await?;
            let res = res.map_err(Error::ErrorResponded)?;
            res
        }};
    }
}

pub(crate) mod core {
    mod connection;
    mod driver;
    mod event_emitter;
    mod message;
    mod remote_object;
    mod transport;
    pub use connection::*;
    pub use driver::*;
    pub use event_emitter::*;
    pub use message::*;
    pub(crate) use remote_object::*;
    pub use transport::*;
}

pub(crate) mod browser_type;
pub(crate) mod playwright;
pub(crate) mod selectors;
pub(crate) mod utils;

pub(crate) mod artifact;
pub(crate) mod binding_call;
pub(crate) mod browser;
pub(crate) mod browser_context;
pub(crate) mod console_message;
pub(crate) mod dialog;
pub(crate) mod download;
pub(crate) mod element_handle;
pub(crate) mod file_hooser;
pub(crate) mod frame;
pub(crate) mod js_handle;
pub(crate) mod page;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod route;
pub(crate) mod stream;
pub(crate) mod video;
pub(crate) mod websocket;
pub(crate) mod worker;

//_accessibility.py
//_api_structures.py
//_api_types.py
//_element_handle.py
//_event_context_manager.py
//_helper.py
//_impl_to_api_mapping.py
//_input.py
//_path_utils.py
//_sync_base.py
//_video.py
//_wait_helper.py
// ChromiumBrowserContext CdpSession
