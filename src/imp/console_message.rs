use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct ConsoleMessage {
    channel: ChannelOwner
}

impl ConsoleMessage {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for ConsoleMessage {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
