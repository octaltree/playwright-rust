use crate::imp::{core::*, page::Page, prelude::*, utils::StorageState};

#[derive(Debug)]
pub(crate) struct BrowserContext {
    channel: ChannelOwner,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    pages: Vec<Weak<Page>>
}

impl BrowserContext {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {} = serde_json::from_value(channel.initializer.clone())?;
        let var = Mutex::new(Variable::default());
        Ok(Self { channel, var })
    }

    pub(crate) fn pages(&self) -> Vec<Weak<Page>> { self.var.lock().unwrap().pages.clone() }

    pub(crate) async fn new_page(&self) -> Result<Weak<Page>, Arc<Error>> {
        let res = send_message!(self, "newPage", Map::new());
        let guid = only_guid(&res)?;
        let p = find_object!(self.context()?.lock().unwrap(), &guid, Page)?;
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

    // TODO: def set_default_navigation_timeout(self, timeout: float) -> None:
    // TODO: def set_default_timeout(self, timeout: float) -> None:
    // TODO: def browser(self) -> Optional["Browser"]:
    // TODO: async def cookies(self, urls: Union[str, List[str]] = None) -> List[Cookie]:
    // TODO: async def add_cookies(self, cookies: List[Cookie]) -> None:
    // TODO: async def clear_cookies(self) -> None:
    // TODO: async def grant_permissions(
    // TODO: async def clear_permissions(self) -> None:
    // TODO: async def set_geolocation(self, geolocation: Geolocation = None) -> None:
    // TODO: async def set_extra_http_headers(self, headers: Dict[str, str]) -> None:
    // TODO: async def set_offline(self, offline: bool) -> None:
    // TODO: async def add_init_script(
    // TODO: async def expose_binding(
    // TODO: async def expose_function(self, name: str, callback: Callable) -> None:
    // TODO: async def route(self, url: URLMatch, handler: RouteHandler) -> None:
    // TODO: async def unroute(
    // TODO: def expect_event(
    // TODO: async def close(self) -> None:
    // TODO: async def storage_state(self, path: Union[str, Path] = None) -> StorageState:
    // TODO: async def wait_for_event(
    // TODO: def expect_page(
}

impl RemoteObject for BrowserContext {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
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
    });
}
