use crate::{
    api::{browser::ContextBuilder, browser_type::PersistentContextLauncher},
    imp::{
        browser_type::BrowserType, core::*, impl_future::*, prelude::*, selectors::Selectors,
        utils::Viewport
    }
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
        let i: Initializer = serde_json::from_value(channel.initializer.clone())?;
        let chromium = get_object!(ctx, &i.chromium.guid, BrowserType)?;
        let firefox = get_object!(ctx, &i.firefox.guid, BrowserType)?;
        let webkit = get_object!(ctx, &i.webkit.guid, BrowserType)?;
        let selectors = get_object!(ctx, &i.selectors.guid, Selectors)?;
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

    pub(crate) fn device(&self, name: &str) -> Option<&DeviceDescriptor> {
        self.devices.iter().find(|d| d.name == name)
    }

    pub(crate) fn chromium(&self) -> Weak<BrowserType> { self.chromium.clone() }

    pub(crate) fn firefox(&self) -> Weak<BrowserType> { self.firefox.clone() }

    pub(crate) fn webkit(&self) -> Weak<BrowserType> { self.webkit.clone() }

    pub(crate) fn selectors(&self) -> Weak<Selectors> { self.selectors.clone() }

    pub(crate) fn wait_initial_object(conn: &Connection) -> WaitInitialObject {
        WaitInitialObject::new(conn.context())
    }
}

impl RemoteObject for Playwright {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    chromium: OnlyGuid,
    firefox: OnlyGuid,
    webkit: OnlyGuid,
    android: OnlyGuid,
    selectors: OnlyGuid,
    device_descriptors: Vec<DeviceDescriptor>
}

pub(crate) struct WaitInitialObject {
    ctx: Wm<Context>,
    started: Instant
}

impl WaitInitialObject {
    fn new(ctx: Wm<Context>) -> Self {
        Self {
            ctx,
            started: Instant::now()
        }
    }
}

impl Future for WaitInitialObject {
    type Output = Result<Weak<Playwright>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let i: &S<Guid> = S::validate("Playwright").unwrap();
        let this = self.get_mut();
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                if this.started.elapsed().as_secs() > 10 {
                    return Poll::Ready(Err(Error::InitializationError));
                }
                return Poll::Pending;
            }};
        }
        let rc = upgrade(&this.ctx)?;
        let c = match rc.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => pending!(),
            Err(e) => Err(e).unwrap()
        };
        match get_object!(c, i, Playwright) {
            Ok(p) => Poll::Ready(Ok(p)),
            Err(_) => pending!()
        }
    }
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
            screen: Option<Viewport>,
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
                    screen,
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
            screen,
            device_scale_factor,
            is_mobile,
            has_touch,
            default_browser_type
        })
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
