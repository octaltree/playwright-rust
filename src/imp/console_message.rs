use crate::imp::{core::*, js_handle::JsHandle, prelude::*, utils::SourceLocation};

#[derive(Debug)]
pub(crate) struct ConsoleMessage {
    channel: ChannelOwner,
    location: SourceLocation,
    args: Vec<Weak<JsHandle>>
}

impl ConsoleMessage {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        #[derive(Deserialize)]
        struct De {
            location: SourceLocation,
            args: Vec<Str<Guid>>
        }
        let De { location, args } = serde_json::from_value(channel.initializer.clone())?;
        let args = args
            .iter()
            .map(|g| get_object!(ctx, g, JsHandle))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            channel,
            location,
            args
        })
    }

    pub(crate) fn r#type(&self) -> &str {
        self.channel()
            .initializer
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
    }

    pub(crate) fn text(&self) -> &str {
        self.channel()
            .initializer
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
    }

    pub(crate) fn location(&self) -> &SourceLocation { &self.location }

    pub(crate) fn args(&self) -> &[Weak<JsHandle>] { &self.args }
}

impl RemoteObject for ConsoleMessage {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
