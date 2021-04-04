use crate::imp::{core::*, prelude::*, websocket::WebSocket as Impl};

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
    fn new(&self, inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }
}
