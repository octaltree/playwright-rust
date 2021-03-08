use crate::imp::{core::*, prelude::*, websocket::WebSocket as Impl};

pub struct WebSocket {
    inner: Weak<Impl>
}

impl WebSocket {
    fn new(&self, inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }
}
