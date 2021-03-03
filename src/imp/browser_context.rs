use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct BrowserContext {
    channel: ChannelOwner,
    name: String
}

impl BrowserContext {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let Initializer { name } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self { channel, name })
    }

    // TODO: def set_default_navigation_timeout(self, timeout: float) -> None:
    // TODO: def set_default_timeout(self, timeout: float) -> None:
    // TODO: def pages(self) -> List[Page]:
    // TODO: def browser(self) -> Optional["Browser"]:
    // TODO: async def new_page(self) -> Page:
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
struct Initializer {
    name: String
}
