use crate::imp::{
    browser::Browser,
    browser_context::BrowserContext,
    core::*,
    prelude::*,
    utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, Viewport}
};

#[derive(Debug)]
pub(crate) struct BrowserType {
    channel: ChannelOwner,
    name: String,
    executable: PathBuf
}

impl BrowserType {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let Initializer { name, executable } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            name,
            executable
        })
    }

    pub(crate) fn name(&self) -> &str { &self.name }

    pub(crate) fn executable(&self) -> &Path { &self.executable }

    pub(crate) async fn launch(
        &self,
        args: LaunchArgs<'_, '_, '_>
    ) -> Result<Weak<Browser>, Arc<ConnectionError>> {
        let m: Str<Method> = "launch".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        let LaunchResponse {
            browser: OnlyGuid { guid }
        } = serde_json::from_value((*res).clone()).map_err(ConnectionError::Serde)?;
        let b = find_object!(self.context()?.lock().unwrap(), &guid, Browser)?;
        Ok(b)
    }

    pub(crate) async fn launch_persistent_context(
        &self,
        args: LaunchPersistentContextArgs<'_, '_, '_, '_, '_, '_, '_>
    ) -> Result<Weak<BrowserContext>, Arc<ConnectionError>> {
        let m: Str<Method> = "launchPersistentContext".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        let LaunchPersistentContextResponse {
            context: OnlyGuid { guid }
        } = serde_json::from_value((*res).clone()).map_err(ConnectionError::Serde)?;
        let b = find_object!(self.context()?.lock().unwrap(), &guid, BrowserContext)?;
        Ok(b)
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchArgs<'a, 'b, 'c> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executablePath")]
    pub(crate) executable: Option<&'a Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) args: Option<&'b [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ignore_all_default_args: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGINT")]
    pub(crate) handle_sigint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGTERM")]
    pub(crate) handle_sigterm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGHUP")]
    pub(crate) handle_sighup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) env: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) headless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) devtools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) proxy: Option<ProxySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downloadsPath")]
    pub(crate) downloads: Option<&'c Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "slowMo")]
    pub(crate) slowmo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chromium_sandbox: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) firefox_user_prefs: Option<Map<String, Value>>
}

impl RemoteObject for BrowserType {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    name: String,
    #[serde(rename = "executablePath")]
    executable: PathBuf
}

#[derive(Deserialize)]
struct LaunchResponse {
    browser: OnlyGuid
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchPersistentContextArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    user_data_dir: &'a Path,
    sdk_language: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executablePath")]
    pub(crate) executable: Option<&'a Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) args: Option<&'b [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ignore_all_default_args: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGINT")]
    pub(crate) handle_sigint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGTERM")]
    pub(crate) handle_sigterm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleSIGHUP")]
    pub(crate) handle_sighup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) env: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) headless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) devtools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) proxy: Option<ProxySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downloadsPath")]
    pub(crate) downloads: Option<&'c Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "slowMo")]
    pub(crate) slowmo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) viewport: Option<Viewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_viewport: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreHTTPSErrors")]
    pub(crate) ignore_http_errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "javaScriptEnabled")]
    pub(crate) js_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bypassCSP")]
    pub(crate) bypass_csp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<&'c str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) locale: Option<&'c str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timezone_id: Option<&'c str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) geolocation: Option<Geolocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) permissions: Option<&'d [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extraHTTPHeaders")]
    pub(crate) extra_http_headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) http_credentials: Option<&'e HttpCredentials>,
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
    pub(crate) record_video: Option<RecordVideo<'f>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) record_har: Option<RecordHar<'g>>
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecordVideo<'a> {
    dir: &'a Path,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<Viewport>
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecordHar<'a> {
    path: &'a Path,
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_content: Option<bool>
}

impl<'a> LaunchPersistentContextArgs<'a, '_, '_, '_, '_, '_, '_> {
    fn new(user_data_dir: &'a Path) -> Self {
        let sdk_language = "rust";
        Self {
            user_data_dir,
            sdk_language,
            executable: None,
            args: None,
            ignore_all_default_args: None,
            handle_sigint: None,
            handle_sigterm: None,
            handle_sighup: None,
            timeout: None,
            env: None,
            headless: None,
            devtools: None,
            proxy: None,
            downloads: None,
            slowmo: None,
            viewport: None,
            no_viewport: None,
            ignore_http_errors: None,
            js_enabled: None,
            bypass_csp: None,
            user_agent: None,
            locale: None,
            timezone_id: None,
            geolocation: None,
            permissions: None,
            extra_http_headers: None,
            offline: None,
            http_credentials: None,
            device_scale_factor: None,
            is_mobile: None,
            has_touch: None,
            color_scheme: None,
            accept_downloads: None,
            chromium_sandbox: None,
            record_video: None,
            record_har: None
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LaunchPersistentContextResponse {
    context: OnlyGuid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::playwright::Playwright;

    crate::runtime_test!(launch, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let p = Playwright::wait_initial_object(&conn).await.unwrap();
        let p = p.upgrade().unwrap();
        let chromium = p.chromium().upgrade().unwrap();
        let res = chromium.launch(LaunchArgs::default()).await;
        dbg!(&res);
        res.unwrap();
    });

    // crate::runtime_test!(launch_persistent_context, {
    //    let driver = Driver::install().unwrap();
    //    let conn = Connection::run(&driver.executable()).unwrap();
    //    let p = Playwright::wait_initial_object(&conn).await.unwrap();
    //    let p = p.upgrade().unwrap();
    //    let firefox = p.firefox().upgrade().unwrap();
    //    let res = firefox
    //        .launch_persistent_context(LaunchPersistentContextArgs::new(".".as_ref()))
    //        .await;
    //    dbg!(&res);
    //    res.unwrap();
    //});
}
