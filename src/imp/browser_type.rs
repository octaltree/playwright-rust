use crate::imp::{core::*, prelude::*};
use std::path::{Path, PathBuf};

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

    pub(crate) fn executable_path(&self) -> &Path { &self.executable }

    // TODO: Ok Browser
    pub(crate) async fn launch(&self, args: LaunchArgs) -> Result<(), Rc<ConnectionError>> {
        let m: Str<Method> = "launch".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        Ok(())
    }

    // TODO: Ok BrowserContext
    pub(crate) async fn launch_persistent_context(
        &self,
        args: LaunchPersistentContextArgs
    ) -> Result<(), Rc<ConnectionError>> {
        let m: Str<Method> = "launchPersistentContext".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        Ok(())
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArgs {
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
// chromiumSandbox: bool = None,
// firefoxUserPrefs: Dict[str, Union[str, float, bool]] = None,
}

#[derive(Serialize)]
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
