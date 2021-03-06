pub use crate::imp::utils::ProxySettings;
use crate::{
    api::{browser::Browser, browser_context::BrowserContext},
    imp::{self, browser_type::LaunchArgs, core::*, prelude::*},
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

    pub fn launcher(&mut self) -> Launcher<'_, '_, '_> { Launcher::new(self.inner.clone()) }

    // async fn launch_persistent_context(&mut self) -> Result<Browser, Error> {
    //    unimplemented!();
    //}
}

/// [`BrowserType::launcher`]
pub struct Launcher<'a, 'b, 'c> {
    inner: Weak<imp::browser_type::BrowserType>,
    args: LaunchArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> Launcher<'a, 'b, 'c> {
    pub async fn launch(self) -> Result<Browser, Error> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch(args).await?;
        Ok(Browser::new(r))
    }

    fn new(inner: Weak<imp::browser_type::BrowserType>) -> Self {
        Launcher {
            inner,
            args: LaunchArgs::default()
        }
    }

    optional_setter!(executable, &'a Path);
    optional_setter!(args, &'b [String]);
    optional_setter!(ignore_all_default_args, bool);
    optional_setter!(handle_sigint, bool);
    optional_setter!(handle_sigterm, bool);
    optional_setter!(handle_sighup, bool);
    optional_setter!(timeout, f64);
    optional_setter!(env, Map<String, Value>);
    optional_setter!(headless, bool);
    optional_setter!(devtools, bool);
    optional_setter!(proxy, ProxySettings);
    optional_setter!(downloads, &'c Path);
    optional_setter!(slowmo, f64);
    optional_setter!(chromium_sandbox, f64);
    optional_setter!(firefox_user_prefs, Map<String, Value>);
}
