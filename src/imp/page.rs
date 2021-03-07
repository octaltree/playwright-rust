use crate::imp::{core::*, frame::Frame, prelude::*, utils::Viewport};

#[derive(Debug)]
pub(crate) struct Page {
    channel: ChannelOwner,
    viewport: Option<Viewport>,
    main_frame: Weak<Frame>
}

#[derive(Debug)]
pub(crate) struct BindingCall {
    channel: ChannelOwner
}

impl Page {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            main_frame: OnlyGuid { guid },
            viewport
        } = serde_json::from_value(channel.initializer.clone())?;
        let main_frame = find_object!(ctx, &guid, Frame)?;
        Ok(Self {
            channel,
            viewport,
            main_frame
        })
    }

    pub(crate) fn main_frame(&self) -> Weak<Frame> { self.main_frame.clone() }
}

impl BindingCall {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Page {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for BindingCall {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    main_frame: OnlyGuid,
    #[serde(rename = "viewportSIze")]
    viewport: Option<Viewport>
}
