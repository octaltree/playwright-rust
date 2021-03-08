use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct WebSocket {
    channel: ChannelOwner,
    url: String
}

impl WebSocket {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { url } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self { channel, url })
    }
}

impl RemoteObject for WebSocket {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String
}
