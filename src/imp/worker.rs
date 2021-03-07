use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Worker {
    channel: ChannelOwner
}

impl Worker {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Worker {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
