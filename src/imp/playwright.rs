use crate::{
    imp::{browser_type::BrowserType, core::*, prelude::*, selectors::Selectors},
    utils::DeviceDescriptor
};
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner,
    pub(crate) chromium: Rweak<BrowserType>,
    pub(crate) firefox: Rweak<BrowserType>,
    pub(crate) webkit: Rweak<BrowserType>,
    pub(crate) selectors: Rweak<Selectors>,
    pub(crate) devices: Vec<DeviceDescriptor>
}

impl Playwright {
    pub(crate) fn try_new(
        conn: &Connection,
        channel: ChannelOwner
    ) -> Result<Self, ConnectionError> {
        let i: Initializer = serde_json::from_value(channel.initializer.clone())?;
        let chromium = find_object!(conn, &i.chromium.guid, BrowserType)?;
        let firefox = find_object!(conn, &i.firefox.guid, BrowserType)?;
        let webkit = find_object!(conn, &i.webkit.guid, BrowserType)?;
        let selectors = find_object!(conn, &i.selectors.guid, Selectors)?;
        let devices = i.device_descriptors;
        Ok(Self {
            channel,
            chromium,
            firefox,
            webkit,
            selectors,
            devices
        })
    }

    pub(crate) fn devices(&self) -> &[DeviceDescriptor] { &self.devices }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    chromium: RefGuid,
    firefox: RefGuid,
    webkit: RefGuid,
    android: RefGuid,
    selectors: RefGuid,
    device_descriptors: Vec<DeviceDescriptor>
}

#[derive(Debug, Deserialize)]
struct RefGuid {
    guid: Str<Guid>
}
