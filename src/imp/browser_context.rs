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
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    browser: Option<Weak<Browser>>,
    pages: Vec<Weak<Page>>,
    timeout: Option<u32>,
    navigation_timeout: Option<u32>
}

impl BrowserContext {
    const DEFAULT_TIMEOUT: u32 = 30000;

    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {} = serde_json::from_value(channel.initializer.clone())?;
        let browser = match &channel.parent {
            Some(RemoteWeak::Browser(b)) => Some(b.clone()),
            _ => None
        };
        let var = Mutex::new(Variable {
            browser,
            ..Variable::default()
        });
        Ok(Self {
            channel,
            var,
            tx: Mutex::default()
        })
    }

    pub(crate) async fn new_page(&self) -> Result<Weak<Page>, Arc<Error>> {
        let res = send_message!(self, "newPage", Map::new());
        let guid = only_guid(&res)?;
        let p = get_object!(self.context()?.lock(), guid, Page)?;
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
        #[serde(rename_all = "camelCase")]
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
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            cookies: &'a [Cookie]
        }
        let args = Args { cookies };
        let _ = send_message!(self, "addCookies", args);
        Ok(())
    }

    pub(crate) async fn grant_permissions(
        &self,
        permissions: &[String],
        origin: Option<&str>
    ) -> ArcResult<()> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a, 'b> {
            permissions: &'a [String],
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
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
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
        args.insert("source", script);
        let _ = send_message!(self, "addInitScript", args);
        Ok(())
    }

    pub(crate) async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            headers: Vec<Header>
        }
        let args = Args {
            headers: headers.into_iter().map(Header::from).collect()
        };
        let _ = send_message!(self, "setExtraHTTPHeaders", args);
        Ok(())
    }

    // async def expose_binding(
    // async def expose_function(self, name: str, callback: Callable) -> None:
    // async def route(self, url: URLMatch, handler: RouteHandler) -> None:
    // async def unroute(

    // async fn pause(&self) -> ArcResult<()> {
    //    let _ = send_message!(self, "pause", Map::new());
    //    Ok(())
    //}
}

// mutable
impl BrowserContext {
    pub(crate) fn browser(&self) -> Option<Weak<Browser>> { self.var.lock().browser.clone() }

    pub(crate) fn set_browser(&self, browser: Weak<Browser>) {
        self.var.lock().browser = Some(browser);
    }

    pub(crate) fn pages(&self) -> Vec<Weak<Page>> { self.var.lock().pages.clone() }

    pub(super) fn push_page(&self, p: Weak<Page>) { self.var.lock().pages.push(p); }

    pub(super) fn remove_page(&self, page: &Weak<Page>) {
        let pages = &mut self.var.lock().pages;
        pages.remove_one(|p| p.ptr_eq(page));
    }

    pub(crate) fn default_timeout(&self) -> u32 {
        self.var.lock().timeout.unwrap_or(Self::DEFAULT_TIMEOUT)
    }

    pub(crate) fn default_navigation_timeout(&self) -> u32 {
        self.var
            .lock()
            .navigation_timeout
            .unwrap_or(Self::DEFAULT_TIMEOUT)
    }

    pub(crate) async fn set_default_timeout(&self, timeout: u32) -> ArcResult<()> {
        let mut args = Map::new();
        args.insert("timeout".into(), timeout.into());
        let _ = send_message!(self, "setDefaultTimeoutNoReply", args);
        self.var.lock().timeout = Some(timeout);
        Ok(())
    }

    pub(crate) async fn set_default_navigation_timeout(&self, timeout: u32) -> ArcResult<()> {
        let mut args = Map::new();
        args.insert("timeout".into(), timeout.into());
        let _ = send_message!(self, "setDefaultNavigationTimeoutNoReply", args);
        self.var.lock().navigation_timeout = Some(timeout);
        Ok(())
    }

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let browser = match self.browser().and_then(|b| b.upgrade()) {
            None => return Ok(()),
            Some(b) => b
        };
        let this = get_object!(ctx, self.guid(), BrowserContext)?;
        browser.remove_context(&this);
        self.emit_event(Evt::Close);
        Ok(())
    }

    fn on_route(&self, _ctx: &Context, _parmas: Map<String, Value>) -> Result<(), Error> {
        // TODO: noimplemented
        Ok(())
    }
}

impl RemoteObject for BrowserContext {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        ctx: &Context,
        method: Str<Method>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        match method.as_str() {
            "request" => {
                if let Some(page) = self.pages().last() {
                    let first = guid_from_params(params.get("request").unwrap())?;
                    let page = page.upgrade().unwrap();
                    let request = get_object!(ctx, first, Request)?;
                    page.on_request(request)?;
                }
            }
            "response" => {
                if let Some(page) = self.pages().last() {
                    let first = guid_from_params(params.get("response").unwrap())?;
                    let page = page.upgrade().unwrap();
                    let response = get_object!(ctx, first, Response)?;
                    page.on_response(response)?;
                }
            }
            "page" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let p = get_object!(ctx, &guid, Page)?;
                self.push_page(p.clone());
                self.emit_event(Evt::Page(p));
            }
            "close" => self.on_close(ctx)?,
            "bindingCall" => {}
            "route" => self.on_route(ctx, params)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    Close,
    Page(Weak<Page>)
}

impl EventEmitter for BrowserContext {
    type Event = Evt;

    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().clone() }

    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock() = Some(tx); }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Close,
    Page
}

impl IsEvent for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Self::Close => EventType::Close,
            Self::Page(_) => EventType::Page
        }
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
        c.set_default_timeout(30000).await.unwrap();
    });
}
