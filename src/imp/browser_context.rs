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
    browser: Option<Weak<Browser>>,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    pages: Vec<Weak<Page>>
}

impl BrowserContext {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {} = serde_json::from_value(channel.initializer.clone())?;
        let browser = match &channel.parent {
            Some(RemoteWeak::Browser(b)) => Some(b.clone()),
            _ => None
        };
        let var = Mutex::new(Variable::default());
        Ok(Self {
            channel,
            var,
            browser
        })
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

    // def set_default_navigation_timeout(self, timeout: float) -> None:
    // def set_default_timeout(self, timeout: float) -> None:
    // async def expose_binding(
    // async def expose_function(self, name: str, callback: Callable) -> None:
    // async def route(self, url: URLMatch, handler: RouteHandler) -> None:
    // async def unroute(
    // def expect_event(
    // async def close(self) -> None:
    // async def wait_for_event(
    // def expect_page(

    pub(crate) fn browser(&self) -> Option<Weak<Browser>> { self.browser.clone() }
}

// mutable
impl BrowserContext {
    pub(crate) fn pages(&self) -> Vec<Weak<Page>> { self.var.lock().unwrap().pages.clone() }

    pub(super) fn push_page(&self, p: Weak<Page>) { self.var.lock().unwrap().pages.push(p); }

    pub(super) fn remove_page(&self, p: &Weak<Page>) {
        let pages = &mut self.var.lock().unwrap().pages;
        let i = match pages
            .iter()
            .zip(0usize..)
            .find(|(v, _)| v.ptr_eq(p))
            .map(|(_, i)| i)
        {
            None => return,
            Some(i) => i
        };
        pages.remove(i);
    }

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let browser = match self.browser().and_then(|b| b.upgrade()) {
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
                let first = first_object(params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let p = get_object!(ctx, &guid, Page)?;
                self.push_page(p);
            }
            "close" => {
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
    use crate::imp::{browser::*, browser_type::*, playwright::Playwright};

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
