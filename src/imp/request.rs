use crate::imp::{core::*, frame::Frame, prelude::*};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner,
    url: String,
    resource_type: String,
    method: String,
    is_navigation_request: bool,
    frame: Weak<Frame>,
    post_data: Option<String>
}

impl Request {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            resource_type,
            method,
            frame,
            is_navigation_request,
            post_data
        } = serde_json::from_value(channel.initializer.clone())?;
        let frame = find_object!(ctx, &frame.guid, Frame)?;
        Ok(Self {
            channel,
            url,
            resource_type,
            method,
            frame,
            is_navigation_request,
            post_data
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }

    pub(crate) fn resource_type(&self) -> &str { &self.resource_type }

    pub(crate) fn method(&self) -> &str { &self.method }

    pub(crate) fn is_navigation_request(&self) -> bool { self.is_navigation_request }

    pub(crate) fn frame(&self) -> Weak<Frame> { self.frame.clone() }

    pub(crate) fn post_data(&self) -> Option<Vec<u8>> {
        base64::decode(self.post_data.as_ref()?).ok()
    }

    pub(crate) fn post_data_as_string(&self) -> Option<String> {
        let bytes = self.post_data()?;
        let s = String::from_utf8(bytes).ok()?;
        Some(s)
    }
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
    is_navigation_request: bool,
    // base64
    post_data: Option<String>
}
