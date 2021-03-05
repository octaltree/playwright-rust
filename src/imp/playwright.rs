use crate::{
    api::utils::DeviceDescriptor,
    imp::{browser_type::BrowserType, core::*, impl_future::*, prelude::*, selectors::Selectors}
};
use serde::Deserialize;
use std::sync::TryLockError;

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
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let i: Initializer = serde_json::from_value(channel.initializer.clone())?;
        let chromium = find_object!(ctx, &i.chromium.guid, BrowserType)?;
        let firefox = find_object!(ctx, &i.firefox.guid, BrowserType)?;
        let webkit = find_object!(ctx, &i.webkit.guid, BrowserType)?;
        let selectors = find_object!(ctx, &i.selectors.guid, Selectors)?;
        let devices = i.device_descriptors;
        Ok(Self {
            channel,
            devices,
            chromium,
            firefox,
            webkit,
            selectors
        })
    }

    pub(crate) fn devices(&self) -> &[DeviceDescriptor] { &self.devices }

    pub(crate) fn chromium(&self) -> Weak<BrowserType> { self.chromium.clone() }

    pub(crate) fn firefox(&self) -> Weak<BrowserType> { self.firefox.clone() }

    pub(crate) fn webkit(&self) -> Weak<BrowserType> { self.webkit.clone() }

    pub(crate) fn selectors(&self) -> Weak<Selectors> { self.selectors.clone() }

    pub(crate) fn wait_initial_object(conn: &Connection) -> WaitInitialObject {
        WaitInitialObject(conn.context())
    }
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

pub(crate) struct WaitInitialObject(Wm<Context>);

impl Future for WaitInitialObject {
    type Output = Result<Weak<Playwright>, ConnectionError>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let i: &S<Guid> = S::validate("Playwright").unwrap();
        // TODO: timeout
        let this = self.get_mut();
        let rc = upgrade(&this.0)?;
        let c = match rc.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            Err(e) => Err(e).unwrap()
        };
        match find_object!(c, i, Playwright) {
            Ok(p) => Poll::Ready(Ok(p)),
            Err(_) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
