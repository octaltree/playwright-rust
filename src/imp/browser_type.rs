use crate::{
    api::browser_type::LaunchArgs,
    imp::{browser::Browser, browser_context::BrowserContext, core::*, prelude::*}
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

    // TODO: builder pattern
    pub(crate) async fn launch(
        &self,
        args: LaunchArgs<'_, '_, '_>
    ) -> Result<Weak<Browser>, Arc<ConnectionError>> {
        let m: Str<Method> = "launch".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        let LaunchResponse {
            browser: OnlyGuid { guid }
        } = serde_json::from_value((*res).clone()).map_err(ConnectionError::Serde)?;
        let b = find_object!(
            upgrade(&self.channel().ctx)?.lock().unwrap(),
            &guid,
            Browser
        )?;
        Ok(b)
    }

    // TODO: required parameter
    pub(crate) async fn launch_persistent_context(
        &self,
        args: LaunchPersistentContextArgs
    ) -> Result<Weak<BrowserContext>, Arc<ConnectionError>> {
        let m: Str<Method> = "launchPersistentContext".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        let LaunchPersistentContextResponse {
            browser_context: OnlyGuid { guid }
        } = serde_json::from_value((*res).clone()).map_err(ConnectionError::Serde)?;
        let b = find_object!(
            upgrade(&self.channel().ctx)?.lock().unwrap(),
            &guid,
            BrowserContext
        )?;
        Ok(b)
    }
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

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchPersistentContextArgs {
    // userDataDir: Union[str, Path],
// executablePath: Union[str, Path] = None,
// args: List[str] = None,
// ignoreDefaultArgs: Union[bool, List[str]] = None,
// handleSIGINT: bool = None,
// handleSIGTERM: bool = None,
// handleSIGHUP: bool = None,
// timeout: float = None,
// env: Env = None,
// headless: bool = None,
// devtools: bool = None,
// proxy: ProxySettings = None,
// downloadsPath: Union[str, Path] = None,
// slowMo: float = None,
// viewport: ViewportSize = None,
// noViewport: bool = None,
// ignoreHTTPSErrors: bool = None,
// javaScriptEnabled: bool = None,
// bypassCSP: bool = None,
// userAgent: str = None,
// locale: str = None,
// timezoneId: str = None,
// geolocation: Geolocation = None,
// permissions: List[str] = None,
// extraHTTPHeaders: Dict[str, str] = None,
// offline: bool = None,
// httpCredentials: HttpCredentials = None,
// deviceScaleFactor: float = None,
// isMobile: bool = None,
// hasTouch: bool = None,
// colorScheme: ColorScheme = None,
// acceptDownloads: bool = None,
// chromiumSandbox: bool = None,
// recordHarPath: Union[Path, str] = None,
// recordHarOmitContent: bool = None,
// recordVideoDir: Union[Path, str] = None,
// recordVideoSize: ViewportSize = None,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LaunchPersistentContextResponse {
    browser_context: OnlyGuid
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
}
