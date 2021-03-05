use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct ConsoleMnaager {
    channel: ChannelOwner
}

impl ConsoleMnaager {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for ConsoleMnaager {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
