use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct ElementHandle {
    channel: ChannelOwner
}

impl ElementHandle {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for ElementHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
