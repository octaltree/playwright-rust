use crate::imp::{self, core::*, impl_future::*, prelude::*};
use serde_json::value::Value;
use std::{
    any::Any,
    fmt::{self, Debug},
    future::Future,
    pin::Pin,
    sync::TryLockError,
    task::Waker
};

pub(crate) fn upgrade<T>(w: &Weak<T>) -> Result<Arc<T>, Error> {
    w.upgrade().ok_or(Error::ObjectNotFound)
}

pub(crate) fn weak_and_then<T, U, F>(w: &Weak<T>, f: F) -> Weak<U>
where
    F: FnOnce(Arc<T>) -> Weak<U>
{
    let rc = match w.upgrade() {
        None => return Weak::new(),
        Some(rc) => rc
    };
    f(rc)
}

#[derive(Debug, Clone)]
pub(crate) struct ChannelOwner {
    pub(crate) ctx: Weak<Mutex<Context>>,
    pub(crate) parent: Option<RemoteWeak>,
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    pub(crate) initializer: Value
}

impl ChannelOwner {
    pub(crate) fn new(
        ctx: Weak<Mutex<Context>>,
        parent: RemoteWeak,
        typ: Str<ObjectType>,
        guid: Str<Guid>,
        initializer: Value
    ) -> Self {
        Self {
            ctx,
            parent: Some(parent),
            typ,
            guid,
            initializer
        }
    }

    pub(crate) fn new_root() -> Self {
        Self {
            ctx: Weak::new(),
            parent: None,
            typ: Str::validate("".into()).unwrap(),
            guid: Str::validate("".into()).unwrap(),
            initializer: Value::default()
        }
    }

    pub(crate) fn create_request(&self, method: Str<Method>) -> RequestBody {
        RequestBody {
            guid: self.guid.clone(),
            method,
            params: Map::default(),
            place: WaitPlaces::new_empty()
        }
    }

    pub(crate) async fn send_message(
        &self,
        r: RequestBody
    ) -> Result<WaitData<WaitMessageResult>, Error> {
        let wait = WaitData::new();
        let r = r.set_wait(&wait);
        let ctx = upgrade(&self.ctx)?;
        ctx.lock().unwrap().send_message(r)?;
        Ok(wait)
    }
}

#[derive(Debug)]
pub(crate) struct DummyObject {
    channel: ChannelOwner
}

impl DummyObject {
    pub(crate) fn new(channel: ChannelOwner) -> Self { DummyObject { channel } }
}

impl RemoteObject for DummyObject {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug)]
pub(crate) struct RootObject {
    channel: ChannelOwner
}

impl RootObject {
    pub(crate) fn new() -> Self {
        Self {
            channel: ChannelOwner::new_root()
        }
    }
}

impl Default for RootObject {
    fn default() -> Self { Self::new() }
}

impl RemoteObject for RootObject {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

pub(crate) trait RemoteObject: Any + Debug {
    fn channel(&self) -> &ChannelOwner;
    fn channel_mut(&mut self) -> &mut ChannelOwner;

    fn guid(&self) -> &S<Guid> { &self.channel().guid }
    fn context(&self) -> Result<Arc<Mutex<Context>>, Error> { upgrade(&self.channel().ctx) }
}

macro_rules! remote_enum {
    ($t:ident, $p: ident) => {
        #[derive(Debug, Clone)]
        pub(crate) enum $t {
            Dummy($p<DummyObject>),
            Root($p<RootObject>),
            BrowserType($p<imp::browser_type::BrowserType>),
            Selectors($p<imp::selectors::Selectors>),
            Browser($p<imp::browser::Browser>),
            BrowserContext($p<imp::browser_context::BrowserContext>),
            Page($p<imp::page::Page>),
            Frame($p<imp::frame::Frame>),
            Response($p<imp::response::Response>),
            Request($p<imp::request::Request>),
            Route($p<imp::route::Route>),
            WebSocket($p<imp::websocket::WebSocket>),
            Worker($p<imp::worker::Worker>),
            Dialog($p<imp::dialog::Dialog>),
            Download($p<imp::download::Download>),
            ConsoleMessage($p<imp::console_message::ConsoleMessage>),
            CdpSession($p<imp::cdp_session::CdpSession>),
            JsHandle($p<imp::js_handle::JsHandle>),
            ElementHandle($p<imp::element_handle::ElementHandle>),
            Playwright($p<imp::playwright::Playwright>)
        }
    };
}

remote_enum! {RemoteArc, Arc}

remote_enum! {RemoteWeak, Weak}

impl RemoteArc {
    pub(crate) fn downgrade(&self) -> RemoteWeak {
        macro_rules! downgrade {
            ($($t:ident),*) => {
                match self {
                    $(
                        Self::$t(x) => RemoteWeak::$t(Arc::downgrade(x))
                    ),*
                }
            }
        }
        downgrade!(
            Dummy,
            Root,
            BrowserType,
            Selectors,
            Browser,
            BrowserContext,
            Page,
            Frame,
            Response,
            Request,
            Route,
            WebSocket,
            Worker,
            Dialog,
            Download,
            ConsoleMessage,
            CdpSession,
            JsHandle,
            ElementHandle,
            Playwright
        )
    }

    pub(crate) fn try_new(
        typ: &S<ObjectType>,
        ctx: &Context,
        c: ChannelOwner
    ) -> Result<RemoteArc, Error> {
        let r = match typ.as_str() {
            "Playwright" => {
                RemoteArc::Playwright(Arc::new(imp::playwright::Playwright::try_new(ctx, c)?))
            }
            "Selectors" => RemoteArc::Selectors(Arc::new(imp::selectors::Selectors::new(c))),
            "BrowserType" => {
                RemoteArc::BrowserType(Arc::new(imp::browser_type::BrowserType::try_new(c)?))
            }
            "Browser" => RemoteArc::Browser(Arc::new(imp::browser::Browser::try_new(c)?)),
            "BrowserContext" => RemoteArc::BrowserContext(Arc::new(
                imp::browser_context::BrowserContext::try_new(c)?
            )),
            "Page" => RemoteArc::Page(Arc::new(imp::page::Page::try_new(ctx, c)?)),
            "Frame" => RemoteArc::Frame(Arc::new(imp::frame::Frame::new(c))),
            "Response" => RemoteArc::Response(Arc::new(imp::response::Response::try_new(ctx, c)?)),
            "Request" => RemoteArc::Request(Arc::new(imp::request::Request::try_new(ctx, c)?)),
            "Route" => RemoteArc::Route(Arc::new(imp::route::Route::new(c))),
            "WebSocket" => RemoteArc::WebSocket(Arc::new(imp::websocket::WebSocket::new(c))),
            "Worker" => RemoteArc::Worker(Arc::new(imp::worker::Worker::new(c))),
            "Dialog" => RemoteArc::Dialog(Arc::new(imp::dialog::Dialog::new(c))),
            "Download" => RemoteArc::Download(Arc::new(imp::download::Download::new(c))),
            "ConsoleMessage" => {
                RemoteArc::ConsoleMessage(Arc::new(imp::console_message::ConsoleMessage::new(c)))
            }
            "CdpSession" => RemoteArc::CdpSession(Arc::new(imp::cdp_session::CdpSession::new(c))),
            "JsHandle" => RemoteArc::JsHandle(Arc::new(imp::js_handle::JsHandle::try_new(ctx, c)?)),
            "ElementHandle" => {
                RemoteArc::ElementHandle(Arc::new(imp::element_handle::ElementHandle::new(c)))
            }
            _ => RemoteArc::Dummy(Arc::new(DummyObject::new(c)))
        };
        Ok(r)
    }
}

pub(crate) struct RequestBody {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>,
    pub(crate) place: WaitPlaces<WaitMessageResult>
}

impl RequestBody {
    pub(crate) fn set_method(mut self, method: Str<Method>) -> Self {
        self.method = method;
        self
    }

    pub(crate) fn set_params(mut self, params: Map<String, Value>) -> Self {
        self.params = params;
        self
    }

    pub(crate) fn set_args<T: Serialize>(mut self, body: T) -> Result<Self, Error> {
        let v = serde_json::value::to_value(body).map_err(Error::Serde)?;
        let p = match v {
            Value::Object(m) => m,
            _ => return Err(Error::NotObject)
        };
        log::debug!("set request {:?}", &p);
        self.params = p;
        Ok(self)
    }

    pub(crate) fn set_wait(mut self, wait: &WaitData<WaitMessageResult>) -> Self {
        self.place = wait.place();
        self
    }

    pub(crate) fn set_guid(mut self, guid: Str<Guid>) -> Self {
        self.guid = guid;
        self
    }
}

pub(crate) type WaitMessageResult = Result<Result<Arc<Value>, Arc<ErrorMessage>>, Arc<Error>>;

#[derive(Debug, Clone)]
pub(crate) struct WaitPlaces<T> {
    pub(crate) value: Wm<Option<T>>,
    pub(crate) waker: Wm<Option<Waker>>
}

pub(crate) struct WaitData<T> {
    place: Arc<Mutex<Option<T>>>,
    waker: Arc<Mutex<Option<Waker>>>
}

impl<T> WaitPlaces<T> {
    pub(crate) fn new_empty() -> Self {
        Self {
            value: Weak::new(),
            waker: Weak::new()
        }
    }
}

impl<T> WaitData<T> {
    pub(crate) fn new() -> Self {
        let place = Arc::new(Mutex::new(None));
        let waker = Arc::new(Mutex::new(None));
        Self { place, waker }
    }

    pub(crate) fn place(&self) -> WaitPlaces<T> {
        let wp = Arc::downgrade(&self.place);
        let ww = Arc::downgrade(&self.waker);
        WaitPlaces {
            value: wp,
            waker: ww
        }
    }
}

impl<T> Future for WaitData<T>
where
    T: Clone
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        log::trace!("poll WaitData");
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
        {
            let x = match this.place.try_lock() {
                Ok(x) => x,
                Err(TryLockError::WouldBlock) => pending!(),
                Err(e) => Err(e).unwrap()
            };
            if let Some(x) = &*x {
                return Poll::Ready(x.clone());
            }
        }
        {
            let mut x = match this.waker.try_lock() {
                Ok(x) => x,
                Err(TryLockError::WouldBlock) => pending!(),
                Err(e) => Err(e).unwrap()
            };
            if x.is_none() {
                *x = Some(cx.waker().clone());
            }
        }
        Poll::Pending
    }
}
