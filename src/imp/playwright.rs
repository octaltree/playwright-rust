use crate::imp::remote_object::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Playwright {
    channel: Arc<ChannelOwner>
}

impl Playwright {
    fn new(channel: Arc<ChannelOwner>) -> Self { Self { channel } }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
}
