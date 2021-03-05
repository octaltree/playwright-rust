use crate::{
    imp::{core::*, prelude::*},
    utils::DeviceDescriptor
};
use serde::Deserialize;
use std::sync::TryLockError;

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner, /* pub(crate) chromium: Weak<BrowserType>,
                            * pub(crate) firefox: Weak<BrowserType>,
                            * pub(crate) webkit: Weak<BrowserType>,
                            * pub(crate) selectors: Weak<Selectors>, */
    pub(crate) devices: Vec<DeviceDescriptor>
}

impl Playwright {
    pub(crate) fn try_new(conn: &Context, channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let i: Initializer = serde_json::from_value(channel.initializer.clone())?;
        // let chromium = find_object!(conn, &i.chromium.guid, BrowserType)?;
        // let firefox = find_object!(conn, &i.firefox.guid, BrowserType)?;
        // let webkit = find_object!(conn, &i.webkit.guid, BrowserType)?;
        // let selectors = find_object!(conn, &i.selectors.guid, Selectors)?;
        let devices = i.device_descriptors;
        Ok(Self {
            channel,
            devices /* chromium,
                     * firefox,
                     * webkit,
                     * selectors,
                     * devices */
        })
    }

    pub(crate) fn devices(&self) -> &[DeviceDescriptor] { &self.devices }

    // pub(crate) fn wait_initial_object(ctx: Wm<Context>) -> WaitInitialObject {
    //    WaitInitialObject(ctx)
    //}
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

// pub(crate) struct WaitInitialObject(Wm<Context>);

// impl Future for WaitInitialObject {
//    type Output = Result<Weak<Playwright>, ConnectionError>;

//    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
//        log::trace!("poll WaitInitialObject");
//        let i: &S<Guid> = S::validate("Playwright").unwrap();
//        // TODO: timeout
//        let this = self.get_mut();
//        let rc = upgrade(&this.0)?;
//        let c = match rc.try_lock() {
//            Ok(x) => x,
//            Err(TryLockError::WouldBlock) => {
//                cx.waker().wake_by_ref();
//                return Poll::Pending;
//            }
//            Err(e) => Err(e).unwrap()
//        };
//        log::trace!("foo");
//        match find_object!(c, i, Playwright) {
//            Ok(p) => Poll::Ready(Ok(p)),
//            Err(_) => {
//                cx.waker().wake_by_ref();
//                Poll::Pending
//            }
//        }
//    }
//}
