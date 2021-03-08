use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Route {
    channel: ChannelOwner
}

impl Route {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { request } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self { channel })
    }
}

impl RemoteObject for Route {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    request: OnlyGuid
}
