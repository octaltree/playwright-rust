use crate::imp::core::*;

#[derive(Debug)]
pub(crate) struct Dialog {
    channel: ChannelOwner
}

impl Dialog {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Dialog {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
