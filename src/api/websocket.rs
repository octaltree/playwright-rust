pub use crate::imp::websocket::Buffer;
use crate::imp::{
    core::*,
    prelude::*,
    websocket::{Evt, WebSocket as Impl}
};

#[derive(Clone)]
pub struct WebSocket {
    inner: Weak<Impl>
}

impl PartialEq for WebSocket {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl WebSocket {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    /// Contains the URL of the WebSocket.
    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }

    pub fn is_closed(&self) -> Result<bool, Error> { Ok(upgrade(&self.inner)?.is_closed()) }

    subscribe_event! {}
}

#[derive(Debug)]
pub enum Event {
    FrameSent(Buffer),
    FrameReceived(Buffer),
    Error(Value),
    Close
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Self {
        match e {
            Evt::FrameSent(x) => Self::FrameSent(x),
            Evt::FrameReceived(x) => Self::FrameReceived(x),
            Evt::Error(x) => Self::Error(x),
            Evt::Close => Self::Close
        }
    }
}
