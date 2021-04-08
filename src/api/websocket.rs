use crate::imp::{
    core::*,
    prelude::*,
    websocket::{Evt, WebSocket as Impl}
};

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

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }

    pub fn is_closed(&self) -> Result<bool, Error> { Ok(upgrade(&self.inner)?.is_closed()) }
}

#[derive(Debug)]
pub(crate) enum Event {
    FrameSent,
    FrameReceived,
    Error,
    Close
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Self {
        match e {
            Evt::FrameSent => Self::FrameSent,
            Evt::FrameReceived => Self::FrameReceived,
            Evt::Error => Self::Error,
            Evt::Close => Self::Close
        }
    }
}
