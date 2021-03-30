pub use crate::imp::browser_type::{RecordHar, RecordVideo};
use crate::{
    api::BrowserContext,
    imp::{
        self,
        browser::NewContextArgs,
        core::*,
        prelude::*,
        utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, StorageState, Viewport}
    },
    Error
};

pub struct Browser {
    inner: Weak<imp::browser::Browser>
}

impl Browser {
    pub(crate) fn new(inner: Weak<imp::browser::Browser>) -> Self { Self { inner } }

    pub fn contexts(&self) -> Result<Vec<BrowserContext>, Error> {
        Ok(upgrade(&self.inner)?
            .contexts()
            .iter()
            .cloned()
            .map(BrowserContext::new)
            .collect())
    }

    pub fn version(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.version().to_owned())
    }

    pub fn exists(&self) -> bool { self.inner.upgrade().is_some() }

    /// new_context [`BrowserContext`]
    pub fn context_builder(&self) -> ContextBuilder<'_, '_, '_, '_, '_, '_, '_> {
        ContextBuilder::new(self.inner.clone())
    }

    /// All temporary browsers will be closed when the connection is terminated, but
    /// it needs to be called explicitly to close it at any given time.
    pub async fn close(&self) -> Result<(), Arc<Error>> {
        let inner = match self.inner.upgrade() {
            None => return Ok(()),
            Some(inner) => inner
        };
        inner.close().await
    }
}

// TODO: async drop

/// [`Browser::context_builder`]
pub struct ContextBuilder<'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    inner: Weak<imp::browser::Browser>,
    args: NewContextArgs<'e, 'f, 'g, 'h, 'i, 'j, 'k>
}

impl<'e, 'f, 'g, 'h, 'i, 'j, 'k> ContextBuilder<'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    pub async fn build(self) -> Result<BrowserContext, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.new_context(args).await?;
        Ok(BrowserContext::new(r))
    }

    fn new(inner: Weak<imp::browser::Browser>) -> Self {
        Self {
            inner,
            args: NewContextArgs::default()
        }
    }

    optional_setter!(
        proxy, ProxySettings;
        viewport, Viewport;
        no_default_viewport, bool;
        ignore_https_errors, bool;
        js_enabled, bool;
        bypass_csp, bool;
        user_agent, &'e str;
        locale, &'f str;
        timezone_id, &'g str;
        geolocation, Geolocation;
        permissions, &'h [String];
        extra_http_headers, HashMap<String, String>;
        offline, bool;
        http_credentials, &'i HttpCredentials;
        device_scale_factor, f64;
        is_mobile, bool;
        has_touch, bool;
        color_scheme, ColorScheme;
        accept_downloads, bool;
        chromium_sandbox, bool;
        record_video, RecordVideo<'j>;
        record_har, RecordHar<'k>;
        storage_state, StorageState);
}
