use crate::imp::{
    browser_context::BrowserContext,
    browser_type::{RecordHar, RecordVideo},
    core::*,
    prelude::*,
    utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, StorageState, Viewport}
};

#[derive(Debug)]
pub(crate) struct Browser {
    channel: ChannelOwner,
    version: String,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    contexts: Vec<Weak<BrowserContext>>
}

impl Browser {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { version } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            version,
            var: Mutex::new(Variable {
                contexts: Vec::new()
            })
        })
    }
    pub(crate) fn version(&self) -> &str { &self.version }

    pub(crate) async fn close(&self) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "close", Map::new());
        Ok(())
    }

    // Responds newtype `OwnerPage` of `SinglePageBrowserContext`.
    // There are different behavior in BrowserContext::new_page
    // pub(crate) async fn new_page(
    //    &self,
    //    args: NewContextArgs<'_, '_, '_, '_, '_, '_, '_>
    //) -> Result<Weak<Page>, Arc<Error>> {
    //    let context = self.new_context(args).await?;
    //    unimplemented!()
    //}
}

// mutable
impl Browser {
    pub(crate) fn contexts(&self) -> Vec<Weak<BrowserContext>> {
        self.var.lock().unwrap().contexts.to_owned()
    }

    fn push_context(&self, c: Weak<BrowserContext>) { self.var.lock().unwrap().contexts.push(c); }

    pub(super) fn remove_context(&self, c: &Weak<BrowserContext>) {
        let contexts = &mut self.var.lock().unwrap().contexts;
        let i = match contexts
            .iter()
            .zip(0usize..)
            .find(|(v, _)| v.ptr_eq(c))
            .map(|(_, i)| i)
        {
            None => return,
            Some(i) => i
        };
        contexts.remove(i);
    }

    pub(crate) async fn new_context(
        &self,
        args: NewContextArgs<'_, '_, '_, '_, '_, '_, '_>
    ) -> Result<Weak<BrowserContext>, Arc<Error>> {
        let res = send_message!(self, "newContext", args);
        let guid = only_guid(&res)?;
        let c = get_object!(self.context()?.lock().unwrap(), &guid, BrowserContext)?;
        self.register_new_context(c.clone())?;
        Ok(c)
    }

    fn register_new_context(&self, c: Weak<BrowserContext>) -> Result<(), Arc<Error>> {
        self.push_context(c);
        // TODO: options
        // let this = get_object!(self.context()?.lock().unwrap(), &self.guid(), Browser)?;
        // let bc = upgrade(&c)?;
        // bc._options = params
        Ok(())
    }
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

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewContextArgs<'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    sdk_language: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) proxy: Option<ProxySettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) viewport: Option<Viewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_default_viewport: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ignore_https_errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "javaScriptEnabled")]
    pub(crate) js_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bypassCSP")]
    pub(crate) bypass_csp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<&'e str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) locale: Option<&'f str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timezone_id: Option<&'g str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) geolocation: Option<Geolocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) permissions: Option<&'h [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extraHTTPHeaders")]
    pub(crate) extra_http_headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) http_credentials: Option<&'i HttpCredentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) device_scale_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_mobile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) has_touch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color_scheme: Option<ColorScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accept_downloads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chromium_sandbox: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) record_video: Option<RecordVideo<'j>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) record_har: Option<RecordHar<'k>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) storage_state: Option<StorageState>
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::{browser_type::*, playwright::Playwright};

    crate::runtime_test!(new_context, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let p = Playwright::wait_initial_object(&conn).await.unwrap();
        let p = p.upgrade().unwrap();
        let chromium = p.chromium().upgrade().unwrap();
        let b = chromium.launch(LaunchArgs::default()).await.unwrap();
        let b = b.upgrade().unwrap();
        b.new_context(NewContextArgs::default()).await.unwrap();
    });
}
