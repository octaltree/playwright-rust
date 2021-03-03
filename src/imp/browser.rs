use crate::imp::{browser_context::BrowserContext, core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Browser {
    channel: ChannelOwner,
    version: String,
    contexts: Vec<Rweak<BrowserContext>>
}

impl Browser {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let Initializer { version } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            version,
            contexts: Vec::new()
        })
    }

    pub(crate) fn contexts(&self) -> &[Rweak<BrowserContext>] { &self.contexts }
    pub(crate) fn version(&self) -> &str { &self.version }

    pub(crate) async fn close(&self) -> Result<(), Rc<ConnectionError>> {
        // TODO: safe close error
        let m: Str<Method> = "close".to_owned().try_into().unwrap();
        #[derive(Serialize)]
        struct CloseArgs {}
        let args = CloseArgs {};
        let _ = send_message!(self, m, args);
        Ok(())
    }

    // TODO: new_context
    // TODO: new_page
}

impl RemoteObject for Browser {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    version: String
}
