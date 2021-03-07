use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner
}

#[derive(Debug)]
pub(crate) struct Route {
    channel: ChannelOwner
}

#[derive(Debug)]
pub(crate) struct WebSocket {
    channel: ChannelOwner
}

impl Request {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl Route {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl WebSocket {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Request {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for Route {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for WebSocket {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
