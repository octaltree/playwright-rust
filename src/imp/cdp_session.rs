use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct CdpSession {
    channel: ChannelOwner
}

impl CdpSession {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for CdpSession {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
