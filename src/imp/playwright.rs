use crate::imp::{
    browser_type::BrowserType,
    connection::{Connection, ConnectionError},
    message::Guid,
    prelude::*,
    remote_object::*,
    selectors::Selectors
};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner,
    chromium: Weak<BrowserType>,
    firefox: Weak<BrowserType>,
    webkit: Weak<BrowserType>,
    selectors: Weak<Selectors>
}

impl Playwright {
    pub(crate) fn try_new(
        conn: &Connection,
        channel: ChannelOwner
    ) -> Result<Self, ConnectionError> {
        // TODO: BrowserType and Selectors from connection
        let i: Initializer = serde_json::from_value(channel.initializer.clone())?;
        // let conn = channel
        //    .conn
        //    .upgrade()
        //    .ok_or(ConnectionError::ObjectNotFound)?;
        // log::trace!("lock playwright");
        // let chromium = conn.lock().unwrap().get_object(&i.chromium.guid);
        // macro_rules! find_object {
        //    ($conn:expr, $guid:expr, $t:ident) => {
        //        match $conn.get_object($guid) {
        //            Some(RemoteWeak::$t(x)) => Ok(x),
        //            _ => Err(ConnectionError::ObjectNotFound)
        //        }
        //    };
        //}
        let chromium = find_object!(conn, &i.chromium.guid, BrowserType)?;
        let firefox = find_object!(conn, &i.firefox.guid, BrowserType)?;
        let webkit = find_object!(conn, &i.webkit.guid, BrowserType)?;
        let selectors = find_object!(conn, &i.selectors.guid, Selectors)?;
        // log::trace!("success playwright");
        Ok(Self {
            channel,
            chromium,
            firefox,
            webkit,
            selectors
        })
    }

    // pub(crate) fn device(&self, name: &str) -> &DeviceDescriptor {}
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
