use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct JsHandle {
    channel: ChannelOwner
}

#[derive(Debug)]
pub(crate) struct ElementHandle {
    channel: ChannelOwner
}

impl JsHandle {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for JsHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
