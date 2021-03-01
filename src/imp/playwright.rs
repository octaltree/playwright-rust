use crate::imp::{prelude::*, remote_object::*};

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner
}

impl Playwright {
    pub(crate) fn new(channel: ChannelOwner) -> Self {
        // TODO: BrowserType and Selectors from connection
        Self { channel }
    }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
