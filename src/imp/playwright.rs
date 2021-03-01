use crate::imp::{connection::ConnectionError, message::Guid, prelude::*, remote_object::*};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner,
    initializer: Initializer
}

impl Playwright {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, ConnectionError> {
        // TODO: BrowserType and Selectors from connection

        let initializer = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            initializer
        })
    }

    // pub(crate) fn device(&self, name: &str) -> &DeviceDescriptor { }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
struct Initializer {
    chromium: RefGuid,
    firefox: RefGuid,
    webkit: RefGuid,
    android: RefGuid,
    selectors: RefGuid,
    deviceDescriptors: Vec<DeviceDescriptor>
}

#[derive(Debug, Deserialize)]
struct RefGuid {
    guid: Str<Guid>
}

#[derive(Debug)]
struct DeviceDescriptor {
    name: String,
    user_agent: String,
    viewport: Viewport,
    device_scale_factor: f64,
    is_mobile: bool,
    has_touch: bool,
    default_browser_type: String
}

impl<'de> Deserialize<'de> for DeviceDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
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
struct Viewport {
    width: i32,
    height: i32
}
