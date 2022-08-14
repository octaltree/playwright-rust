use crate::{
    api::{browser::ContextBuilder, browser_type::PersistentContextLauncher},
    imp::{
        browser_type::BrowserType, core::*, impl_future::*, prelude::*, selectors::Selectors,
        utils::Viewport
    },
    protocol::generated::playwright as protocol
};
use serde::Deserialize;
use std::{sync::TryLockError, time::Instant};

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner,
    chromium: Weak<BrowserType>,
    firefox: Weak<BrowserType>,
    webkit: Weak<BrowserType>,
    selectors: Weak<Selectors>,
    devices: Vec<DeviceDescriptor>
}

impl Playwright {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        // TODO
        let protocol::Initializer {
            android,
            chromium,
            device_descriptors,
            electron,
            firefox,
            pre_launched_browser,
            selectors,
            socks_support,
            utils,
            webkit
        } = serde_json::from_value(channel.initializer.clone())?;
        let chromium = get_object!(ctx, &chromium.guid, BrowserType)?;
        let firefox = get_object!(ctx, &firefox.guid, BrowserType)?;
        let webkit = get_object!(ctx, &webkit.guid, BrowserType)?;
        let selectors = get_object!(ctx, &selectors.guid, Selectors)?;
        let devices = device_descriptors
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<DeviceDescriptor>, ()>>()
            .map_err(|_| Error::InitializationError)?;
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

    pub(crate) fn device(&self, name: &str) -> Option<&DeviceDescriptor> {
        self.devices.iter().find(|d| d.name == name)
    }

    pub(crate) fn chromium(&self) -> Weak<BrowserType> { self.chromium.clone() }

    pub(crate) fn firefox(&self) -> Weak<BrowserType> { self.firefox.clone() }

    pub(crate) fn webkit(&self) -> Weak<BrowserType> { self.webkit.clone() }

    pub(crate) fn selectors(&self) -> Weak<Selectors> { self.selectors.clone() }

    pub(crate) async fn wait_initial_object(conn: &Connection) -> Result<Weak<Self>, Error> {
        let ctx = upgrade(&conn.context())?;
        let ctx = ctx.lock().unwrap();
        let root = get_object!(ctx, &S::validate("").unwrap(), Root)?;
        let root = upgrade(&root)?;
        std::mem::drop(ctx);
        let v = send_message!(
            root,
            "initialize",
            crate::protocol::generated::root::commands::InitializeArgs {
                sdk_language: "python"
            }
        );
        let v = Arc::unwrap_or_clone(v);
        let crate::protocol::generated::root::commands::Initialize {
            playwright: crate::protocol::generated::Playwright { guid }
        } = serde_json::from_value(v)?;
        let ctx = upgrade(&conn.context())?;
        let ctx = ctx.lock().unwrap();
        let p = get_object!(ctx, &guid, Playwright)?;
        Ok(p)
    }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDescriptor {
    pub name: String,
    pub user_agent: String,
    pub viewport: Viewport,
    pub screen: Option<Viewport>,
    pub device_scale_factor: f64,
    pub is_mobile: bool,
    pub has_touch: bool,
    pub default_browser_type: protocol::InitializerDeviceDescriptorsDescriptorDefaultBrowserType
}

impl TryFrom<protocol::InitializerDeviceDescriptors> for DeviceDescriptor {
    type Error = ();
    fn try_from(x: protocol::InitializerDeviceDescriptors) -> Result<Self, Self::Error> {
        let protocol::InitializerDeviceDescriptors {
            name,
            descriptor:
                protocol::InitializerDeviceDescriptorsDescriptor {
                    default_browser_type,
                    device_scale_factor,
                    has_touch,
                    is_mobile,
                    screen,
                    user_agent,
                    viewport
                }
        } = x;
        Ok(DeviceDescriptor {
            name,
            user_agent,
            viewport: viewport.try_into()?,
            screen: if let Some(screen) = screen {
                Some(screen.try_into()?)
            } else {
                None
            },
            device_scale_factor: device_scale_factor.as_f64().ok_or(())?,
            is_mobile,
            has_touch,
            default_browser_type
        })
    }
}

impl TryFrom<protocol::InitializerDeviceDescriptorsDescriptorScreen> for Viewport {
    type Error = ();
    fn try_from(
        x: protocol::InitializerDeviceDescriptorsDescriptorScreen
    ) -> Result<Self, Self::Error> {
        let protocol::InitializerDeviceDescriptorsDescriptorScreen { height, width } = x;
        let width: i32 = width.as_i64().ok_or(())?.try_into().map_err(|_| ())?;
        let height: i32 = height.as_i64().ok_or(())?.try_into().map_err(|_| ())?;
        Ok(Self { width, height })
    }
}

impl TryFrom<protocol::InitializerDeviceDescriptorsDescriptorViewport> for Viewport {
    type Error = ();
    fn try_from(
        x: protocol::InitializerDeviceDescriptorsDescriptorViewport
    ) -> Result<Self, Self::Error> {
        let protocol::InitializerDeviceDescriptorsDescriptorViewport { height, width } = x;
        let width: i32 = width.as_i64().ok_or(())?.try_into().map_err(|_| ())?;
        let height: i32 = height.as_i64().ok_or(())?.try_into().map_err(|_| ())?;
        Ok(Self { width, height })
    }
}

macro_rules! impl_set_device {
    ($device: expr, $builder:expr) => {
        (if let Some(screen) = &$device.screen {
            $builder.screen(screen.clone())
        } else {
            $builder
        })
        .user_agent(&$device.user_agent)
        .viewport(Some($device.viewport.clone()))
        .device_scale_factor($device.device_scale_factor)
        .is_mobile($device.is_mobile)
        .has_touch($device.has_touch)
    };
}

impl DeviceDescriptor {
    pub(crate) fn set_persistent_context<'source, 'b, 'c, 'd, 'e, 'g, 'h, 'i, 'j, 'k, 'l>(
        device: &'source Self,
        builder: PersistentContextLauncher<'b, 'c, 'd, 'e, 'source, 'g, 'h, 'i, 'j, 'k, 'l>
    ) -> PersistentContextLauncher<'b, 'c, 'd, 'e, 'source, 'g, 'h, 'i, 'j, 'k, 'l> {
        impl_set_device!(device, builder)
    }

    pub(crate) fn set_context<'source, 'c, 'd, 'e, 'f, 'g, 'h>(
        device: &'source Self,
        builder: ContextBuilder<'source, 'c, 'd, 'e, 'f, 'g, 'h>
    ) -> ContextBuilder<'source, 'c, 'd, 'e, 'f, 'g, 'h> {
        impl_set_device!(device, builder)
    }
}
