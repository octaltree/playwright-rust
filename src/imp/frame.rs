use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Frame {
    channel: ChannelOwner
}

impl Frame {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Frame {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
