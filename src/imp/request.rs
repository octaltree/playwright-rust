use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner,
    url: String,
    resource_type: String,
    method: String
}

impl Request {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            resource_type,
            method
        } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            url,
            resource_type,
            method
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }

    pub(crate) fn resource_type(&self) -> &str { &self.resource_type }

    pub(crate) fn method(&self) -> &str { &self.method }
}

impl RemoteObject for Request {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String,
    resource_type: String,
    method: String
}
