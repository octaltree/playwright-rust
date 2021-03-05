use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Page {
    channel: ChannelOwner
}

#[derive(Debug)]
pub(crate) struct Worker {
    channel: ChannelOwner
}

#[derive(Debug)]
pub(crate) struct BindingCall {
    channel: ChannelOwner
}

impl Page {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl Worker {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl BindingCall {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Page {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for Worker {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for BindingCall {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
