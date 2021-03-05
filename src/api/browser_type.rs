use crate::{
    api::{browser::Browser, browser_context::BrowserContext},
    imp::{self, core::*, prelude::*},
    Error
};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Weak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Weak<imp::browser_type::BrowserType>) -> Self { Self { inner } }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn name(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.name().into()) }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn executable(&self) -> Result<PathBuf, Error> {
        Ok(upgrade(&self.inner)?.executable().into())
    }

    pub async fn launch(&mut self, args: LaunchArgs<'_, '_, '_>) -> Result<Browser, Error> {
        unimplemented!();
    }

    async fn launch_persistent_context(&mut self) -> Result<Browser, Error> {
        unimplemented!();
    }
}

// TODO: インポート必要なの辛いよね
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArgs<'a, 'b, 'c> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executablePath")]
    executable: Option<&'a Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<&'b [&'c str]> /* ignore_default_args
                                 * ignoreDefaultArgs: Union[bool, List[str]] = None,
                                 * handleSIGINT: bool = None,
                                 * handleSIGTERM: bool = None,
                                 * handleSIGHUP: bool = None,
                                 * timeout: float = None,
                                 * env: Env = None,
                                 * headless: bool = None,
                                 * devtools: bool = None,
                                 * proxy: ProxySettings = None,
                                 * downloadsPath: Union[str, Path] = None,
                                 * slowMo: float = None,
                                 * chromiumSandbox: bool = None,
                                 * firefoxUserPrefs: Dict[str, Union[str, float, bool]] = None, */
}
