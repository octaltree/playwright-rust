use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Response {
    channel: ChannelOwner,
    url: String,
    status: i32,
    status_text: String
}

impl Response {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            status,
            status_text
        } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            url,
            status,
            status_text
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }
    pub(crate) fn status(&self) -> i32 { self.status }
    pub(crate) fn status_text(&self) -> &str { &self.status_text }

    pub(crate) fn ok(&self) -> bool { self.status == 0 || (200..300).contains(&self.status) }
}

impl RemoteObject for Response {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String,
    status: i32,
    status_text: String
}
