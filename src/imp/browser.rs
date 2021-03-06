use crate::imp::{browser_context::BrowserContext, core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Browser {
    channel: ChannelOwner,
    version: String,
    contexts: Vec<Weak<BrowserContext>>
}

impl Browser {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { version } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            version,
            contexts: Vec::new()
        })
    }

    pub(crate) fn contexts(&self) -> &[Weak<BrowserContext>] { &self.contexts }
    pub(crate) fn version(&self) -> &str { &self.version }

    pub(crate) async fn close(&self) -> Result<(), Arc<Error>> {
        let m: Str<Method> = "close".to_owned().try_into().unwrap();
        #[derive(Serialize)]
        struct CloseArgs {}
        let args = CloseArgs {};
        async fn catch(
            this: &Browser,
            m: Str<Method>,
            args: CloseArgs
        ) -> Result<Arc<Value>, Arc<Error>> {
            Ok(send_message!(this, m, args))
        }
        let result = catch(self, m, args).await;
        let err = match result {
            Ok(_) => return Ok(()),
            Err(e) => e
        };
        let _responded_error = match *err {
            Error::ErrorResponded(ref e) => e,
            _ => Err(err)?
        };
        // TODO: has been closed
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
