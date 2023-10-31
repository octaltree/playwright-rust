use crate::imp::{core::*, prelude::*, utils::SourceLocation};

#[derive(Debug)]
pub(crate) struct ConsoleMessage {
    channel: ChannelOwner,
    location: SourceLocation,
    text: String,
    message_type: String
}

impl ConsoleMessage {
    pub(crate) fn try_new(_ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        #[derive(Deserialize)]
        struct De {
            location: SourceLocation,
            text: String,
            r#type: String
        }
        let De {
            location,
            text,
            r#type
        } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            location,
            text,
            message_type: r#type
        })
    }

    pub(crate) fn r#type(&self) -> &str {
        self.channel()
            .initializer
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
    }

    pub(crate) fn text(&self) -> &str { &self.text }

    pub(crate) fn location(&self) -> &SourceLocation { &self.location }

    pub fn message_type(&self) -> &str { &self.message_type }
}

impl RemoteObject for ConsoleMessage {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
