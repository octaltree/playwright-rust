use crate::imp::{
    browser::Browser,
    core::*,
    page::Page,
    prelude::*,
    utils::{Cookie, Geolocation, Header, StorageState}
};

#[derive(Debug)]
pub(crate) struct BrowserContext {
    channel: ChannelOwner,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    pages: Vec<Weak<Page>>,
    browser: Option<Weak<Browser>>
}

impl BrowserContext {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {} = serde_json::from_value(channel.initializer.clone())?;
        let var = Mutex::new(Variable::default());
        Ok(Self { channel, var })
    }

    pub(crate) async fn new_page(&self) -> Result<Weak<Page>, Arc<Error>> {
        let res = send_message!(self, "newPage", Map::new());
        let guid = only_guid(&res)?;
        let p = get_object!(self.context()?.lock().unwrap(), &guid, Page)?;
        Ok(p)
    }

    pub(crate) async fn close(&self) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "close", Map::new());
        Ok(())
    }

    pub(crate) async fn storage_state(&self) -> ArcResult<StorageState> {
        let v = send_message!(self, "storageState", Map::new());
        let s = serde_json::from_value((*v).clone()).map_err(Error::Serde)?;
        Ok(s)
    }

    pub(crate) async fn clear_cookies(&self) -> ArcResult<()> {
        let _ = send_message!(self, "clearCookies", Map::new());
        Ok(())
    }

    pub(crate) async fn cookies(&self, urls: &[String]) -> ArcResult<Vec<Cookie>> {
        #[derive(Serialize)]
        struct Args<'a> {
            urls: &'a [String]
        }
        let args = Args { urls };
        let v = send_message!(self, "cookies", args);
        let cookies = first(&v).ok_or(Error::InvalidParams)?;
        let cs: Vec<Cookie> = serde_json::from_value((*cookies).clone()).map_err(Error::Serde)?;
        Ok(cs)
    }

    pub(crate) async fn add_cookies(&self, cookies: &[Cookie]) -> ArcResult<()> {
        #[derive(Serialize)]
        struct Args<'a> {
            cookies: &'a [Cookie]
        }
        let args = Args { cookies };
        let _ = send_message!(self, "cookies", args);
        Ok(())
    }

    pub(crate) async fn grant_permission(
        &self,
        permissions: &[String],
        origin: Option<&str>
    ) -> ArcResult<()> {
        #[derive(Serialize)]
        struct Args<'a, 'b> {
            permissions: &'a [String],
            #[serde(skip_serializing_if = "Option::is_none")]
            origin: Option<&'b str>
        }
        let args = Args {
            permissions,
            origin
        };
        let _ = send_message!(self, "grantPermissions", args);
        Ok(())
    }

    pub(crate) async fn clear_permissions(&self) -> ArcResult<()> {
        let _ = send_message!(self, "clearPermissions", Map::new());
        Ok(())
    }

    pub(crate) async fn set_geolocation(&self, geolocation: Option<&Geolocation>) -> ArcResult<()> {
        #[derive(Serialize)]
        struct Args<'a> {
            geolocation: Option<&'a Geolocation>
        }
        let args = Args { geolocation };
        let _ = send_message!(self, "setGeolocation", args);
        Ok(())
    }

    pub(crate) async fn set_offline(&self, offline: bool) -> ArcResult<()> {
        let mut args = Map::new();
        args.insert("offline".into(), offline.into());
        let _ = send_message!(self, "setOffline", args);
        Ok(())
    }

    pub(crate) async fn add_init_script(&self, script: &str) -> ArcResult<()> {
        let mut args = HashMap::new();
        args.insert("script", script);
        let _ = send_message!(self, "addInitScript", args);
        Ok(())
    }

    pub(crate) async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        #[derive(Serialize)]
        struct Args {
            headers: Vec<Header>
        }
        let args = Args {
            headers: headers.into_iter().map(Header::from).collect()
        };
        let _ = send_message!(self, "setExtraHTTPHeaders", args);
        Ok(())
    }

    // TODO: def set_default_navigation_timeout(self, timeout: float) -> None:
    // TODO: def set_default_timeout(self, timeout: float) -> None:
    // TODO: def browser(self) -> Optional["Browser"]:
    // TODO: async def expose_binding(
    // TODO: async def expose_function(self, name: str, callback: Callable) -> None:
    // TODO: async def route(self, url: URLMatch, handler: RouteHandler) -> None:
    // TODO: async def unroute(
    // TODO: def expect_event(
    // TODO: async def close(self) -> None:
    // TODO: async def wait_for_event(
    // TODO: def expect_page(
}

// mutable
impl BrowserContext {
    // TODO: set on created
    pub(crate) fn pages(&self) -> Vec<Weak<Page>> { self.var.lock().unwrap().pages.clone() }

    // TODO: set on created
    pub(crate) fn browser(&self) -> Option<Weak<Browser>> {
        self.var.lock().unwrap().browser.clone()
    }

    pub(super) fn set_browser(&self, b: Weak<Browser>) {
        self.var.lock().unwrap().browser = Some(b);
    }

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let b = match self.browser() {
            None => return Ok(()),
            Some(b) => b
        };
        let browser = match b.upgrade() {
            None => return Ok(()),
            Some(b) => b
        };
        let this = get_object!(ctx, &self.guid(), BrowserContext)?;
        browser.remove_context(&this);
        Ok(())
    }
}

impl RemoteObject for BrowserContext {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        ctx: &Context,
        method: &S<Method>,
        params: &Map<String, Value>
    ) -> Result<(), Error> {
        match method.as_str() {
            "page" => {
                // TODO: emit event
                let first = first_object(params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let p = get_object!(ctx, &guid, Page)?;
                self.var.lock().unwrap().pages.push(p);
            }
            "route" => {}
            "bindingCall" => {}
            "close" => {
                // TODO: emit event
                self.on_close(ctx)?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::{browser::*, browser_type::*, core::*, playwright::Playwright};

    crate::runtime_test!(storage_state, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let p = Playwright::wait_initial_object(&conn).await.unwrap();
        let p = p.upgrade().unwrap();
        let chromium = p.chromium().upgrade().unwrap();
        let b = chromium.launch(LaunchArgs::default()).await.unwrap();
        let b = b.upgrade().unwrap();
        let c = b.new_context(NewContextArgs::default()).await.unwrap();
        let c = c.upgrade().unwrap();
        c.storage_state().await.unwrap();
        c.cookies(&[]).await.unwrap();
    });
}
