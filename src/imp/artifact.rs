use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Artifact {
    channel: ChannelOwner
}

impl Artifact {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Artifact {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
