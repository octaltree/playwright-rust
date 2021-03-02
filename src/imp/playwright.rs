use crate::imp::{browser_type::BrowserType, core::*, prelude::*, selectors::Selectors};
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

#[derive(Debug)]
pub struct DeviceDescriptor {
    pub name: String,
    pub user_agent: String,
    pub viewport: Viewport,
    pub device_scale_factor: f64,
    pub is_mobile: bool,
    pub has_touch: bool,
    pub default_browser_type: String
}

impl<'de> Deserialize<'de> for DeviceDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct DeviceDescriptorImpl {
            name: String,
            descriptor: Descriptor
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Descriptor {
            user_agent: String,
            viewport: Viewport,
            device_scale_factor: f64,
            is_mobile: bool,
            has_touch: bool,
            default_browser_type: String
        }
        let DeviceDescriptorImpl {
            name,
            descriptor:
                Descriptor {
                    user_agent,
                    viewport,
                    device_scale_factor,
                    is_mobile,
                    has_touch,
                    default_browser_type
                }
        } = DeviceDescriptorImpl::deserialize(deserializer)?;
        Ok(DeviceDescriptor {
            name,
            user_agent,
            viewport,
            device_scale_factor,
            is_mobile,
            has_touch,
            default_browser_type
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
}
