use crate::{
    imp::{browser_type::BrowserType, core::*, prelude::*, selectors::Selectors},
    utils::DeviceDescriptor
};
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct Playwright {
    channel: ChannelOwner,
    pub(crate) chromium: Weak<BrowserType>,
    pub(crate) firefox: Weak<BrowserType>,
    pub(crate) webkit: Weak<BrowserType>,
    pub(crate) selectors: Weak<Selectors>,
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

pub(crate) struct WaitInitialObject(Weak<Mutex<Connection>>);

impl Future for WaitInitialObject {
    type Output = Result<Weak<Playwright>, ConnectionError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let i: &S<Guid> = S::validate("Playwright").unwrap();
        // TODO: timeout
        let this = self.get_mut();
        let rc = upgrade(&this.0)?;
        let mut c = match rc.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            Err(e) => Err(e).unwrap()
        };
        let p = c.get_object(i);
        match p {
            Some(RemoteWeak::Playwright(p)) => return Poll::Ready(Ok(p)),
            Some(_) => return Poll::Ready(Err(ConnectionError::ObjectNotFound)),
            None => {
                // cx.waker().wake_by_ref();
                // return Poll::Pending;
            }
        }
        let c: Pin<&mut Connection> = Pin::new(&mut c);
        match c.poll_next(cx) {
            Poll::Ready(None) => Poll::Ready(Err(ConnectionError::ReceiverClosed)),
            Poll::Pending => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(())) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
