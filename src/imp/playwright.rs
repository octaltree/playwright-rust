use crate::imp::remote_object::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Playwright {
    channel: ChannelOwner
}

impl Playwright {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
