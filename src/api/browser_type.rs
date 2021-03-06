pub use crate::imp::{
    browser_type::{RecordHar, RecordVideo},
    utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, Viewport}
};
use crate::{
    api::{browser::Browser, browser_context::BrowserContext},
    imp::{
        self,
        browser_type::{LaunchArgs, LaunchPersistentContextArgs},
        core::*,
        prelude::*
    },
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

    pub fn persistent_context_launcher<'a>(
        &mut self,
        user_data_dir: &'a Path
    ) -> PersistentContextLauncher<'a, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        PersistentContextLauncher::new(self.inner.clone(), user_data_dir)
    }
}

/// [`BrowserType::launcher`]
pub struct Launcher<'a, 'b, 'c> {
    inner: Weak<imp::browser_type::BrowserType>,
    args: LaunchArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> Launcher<'a, 'b, 'c> {
    pub async fn launch(self) -> Result<Browser, Arc<Error>> {
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

/// [`BrowserType::persistent_context_launcher`]
pub struct PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    inner: Weak<imp::browser_type::BrowserType>,
    args: LaunchPersistentContextArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    pub async fn launch(self) -> Result<BrowserContext, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch_persistent_context(args).await?;
        Ok(BrowserContext::new(r))
    }

    fn new(inner: Weak<imp::browser_type::BrowserType>, user_data_dir: &'a Path) -> Self {
        Self {
            inner,
            args: LaunchPersistentContextArgs::new(user_data_dir)
        }
    }

    optional_setter!(executable, &'b Path);
    optional_setter!(args, &'c [String]);
    optional_setter!(ignore_all_default_args, bool);
    optional_setter!(handle_sigint, bool);
    optional_setter!(handle_sigterm, bool);
    optional_setter!(handle_sighup, bool);
    optional_setter!(timeout, f64);
    optional_setter!(env, Map<String, Value>);
    optional_setter!(headless, bool);
    optional_setter!(devtools, bool);
    optional_setter!(proxy, ProxySettings);
    optional_setter!(downloads, &'d Path);
    optional_setter!(slowmo, f64);
    optional_setter!(viewport, Viewport);
    optional_setter!(no_default_viewport, bool);
    optional_setter!(ignore_http_errors, bool);
    optional_setter!(js_enabled, bool);
    optional_setter!(bypass_csp, bool);
    optional_setter!(user_agent, &'e str);
    optional_setter!(locale, &'f str);
    optional_setter!(timezone_id, &'g str);
    optional_setter!(geolocation, Geolocation);
    optional_setter!(permissions, &'h [String]);
    optional_setter!(extra_http_headers, HashMap<String, String>);
    optional_setter!(offline, bool);
    optional_setter!(http_credentials, &'i HttpCredentials);
    optional_setter!(device_scale_factor, f64);
    optional_setter!(is_mobile, bool);
    optional_setter!(has_touch, bool);
    optional_setter!(color_scheme, ColorScheme);
    optional_setter!(accept_downloads, bool);
    optional_setter!(chromium_sandbox, bool);
    optional_setter!(record_video, RecordVideo<'j>);
    optional_setter!(record_har, RecordHar<'k>);
}
