use crate::imp::core::*;

#[derive(Debug)]
pub(crate) struct Download {
    channel: ChannelOwner
}

impl Download {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Download {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
