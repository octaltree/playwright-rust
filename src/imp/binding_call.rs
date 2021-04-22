use crate::imp::core::*;

#[derive(Debug)]
pub(crate) struct BindingCall {
    channel: ChannelOwner
}

impl BindingCall {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for BindingCall {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
