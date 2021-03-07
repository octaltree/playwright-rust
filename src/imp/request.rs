use crate::imp::{core::*, frame::Frame, prelude::*};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner,
    url: String,
    resource_type: String,
    method: String,
    is_navigated_request: bool,
    frame: Weak<Frame>
}

impl Request {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            resource_type,
            method,
            frame,
            is_navigated_request
        } = serde_json::from_value(channel.initializer.clone())?;
        let frame = find_object!(ctx, &frame.guid, Frame)?;
        Ok(Self {
            channel,
            url,
            resource_type,
            method,
            frame,
            is_navigated_request
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }

    pub(crate) fn resource_type(&self) -> &str { &self.resource_type }

    pub(crate) fn method(&self) -> &str { &self.method }

    pub(crate) fn is_navigated_request(&self) -> bool { self.is_navigated_request }

    pub(crate) fn frame(&self) -> Weak<Frame> { self.frame.clone() }
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
    method: String,
    frame: OnlyGuid,
    is_navigated_request: bool
}
