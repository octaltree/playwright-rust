use crate::imp::core::*;

#[derive(Debug)]
pub(crate) struct Stream {
    channel: ChannelOwner
}

impl Stream {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Stream {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
