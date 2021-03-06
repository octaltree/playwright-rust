use crate::imp::{browser_context::BrowserContext, core::*, prelude::*, utils::Viewport};

#[derive(Debug)]
pub(crate) struct Browser {
    channel: ChannelOwner,
    version: String,
    contexts: Mutex<Vec<Weak<BrowserContext>>>
}

impl Browser {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { version } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            version,
            contexts: Mutex::new(Vec::new())
        })
    }

    pub(crate) fn contexts(&self) -> Vec<Weak<BrowserContext>> {
        self.contexts.lock().unwrap().to_owned()
    }
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

    pub(crate) async fn new_context(
        &self,
        args: NewContextArgs
    ) -> Result<Weak<BrowserContext>, Arc<Error>> {
        let m: Str<Method> = "newContext".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        let NewContextResponse {
            context: OnlyGuid { guid }
        } = serde_json::from_value((*res).clone()).map_err(Error::Serde)?;
        let c = find_object!(self.context()?.lock().unwrap(), &guid, BrowserContext)?;
        // TODO
        // self._contexts.append(context)
        // context._browser = self
        // context._options = params
        Ok(c)
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewContextArgs {
    sdk_language: &'static str
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewContextResponse {
    context: OnlyGuid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::{browser_type::*, core::*, playwright::Playwright};

    crate::runtime_test!(new_context, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let p = Playwright::wait_initial_object(&conn).await.unwrap();
        let p = p.upgrade().unwrap();
        let chromium = p.chromium().upgrade().unwrap();
        let b = chromium.launch(LaunchArgs::default()).await.unwrap();
        let b = b.upgrade().unwrap();
        b.new_context(NewContextArgs {
            sdk_language: "rust"
        })
        .await;
    });
}
