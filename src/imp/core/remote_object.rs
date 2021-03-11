use crate::imp::{core::*, impl_future::*, prelude::*};
use serde_json::value::Value;
use std::{fmt::Debug, future::Future, pin::Pin, sync::TryLockError, task::Waker};

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

#[derive(Debug)]
pub(crate) struct ChannelOwner {
    pub(crate) ctx: Weak<Mutex<Context>>,
    pub(crate) parent: Option<RemoteWeak>,
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    pub(crate) initializer: Value,
    children: Mutex<Vec<RemoteWeak>>
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
            initializer,
            children: Mutex::new(Vec::new())
        }
    }

    pub(crate) fn new_root() -> Self {
        Self {
            ctx: Weak::new(),
            parent: None,
            typ: Str::validate("".into()).unwrap(),
            guid: Str::validate("".into()).unwrap(),
            initializer: Value::default(),
            children: Mutex::default()
        }
    }

    pub(crate) fn create_request(&self, method: Str<Method>) -> RequestBody {
        RequestBody::new(self.guid.clone(), method)
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

    pub(crate) fn children(&self) -> Vec<RemoteWeak> { self.children.lock().unwrap().to_vec() }

    pub(crate) fn push_child(&self, c: RemoteWeak) {
        let children = &mut self.children.lock().unwrap();
        children.push(c);
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

pub(crate) trait RemoteObject: Debug {
    fn channel(&self) -> &ChannelOwner;
    fn channel_mut(&mut self) -> &mut ChannelOwner;

    fn guid(&self) -> &S<Guid> { &self.channel().guid }
    fn context(&self) -> Result<Arc<Mutex<Context>>, Error> { upgrade(&self.channel().ctx) }

    fn handle_event(
        &self,
        _ctx: &Context,
        _method: &S<Method>,
        _params: &Map<String, Value>
    ) -> Result<(), Error> {
        Ok(())
    }
}

mod remote_enum {
    use super::*;
    use crate::imp::{
        browser::Browser, browser_context::BrowserContext, browser_type::BrowserType,
        console_message::ConsoleMessage, dialog::Dialog, download::Download,
        element_handle::ElementHandle, frame::Frame, js_handle::JsHandle, page::Page,
        playwright::Playwright, request::Request, response::Response, route::Route,
        selectors::Selectors, websocket::WebSocket, worker::Worker
    };

    macro_rules! remote_enum {
        ($t:ident, $p: ident) => {
            #[derive(Debug, Clone)]
            pub(crate) enum $t {
                Dummy($p<DummyObject>),
                Root($p<RootObject>),
                BrowserType($p<BrowserType>),
                Selectors($p<Selectors>),
                Browser($p<Browser>),
                BrowserContext($p<BrowserContext>),
                Page($p<Page>),
                Frame($p<Frame>),
                Response($p<Response>),
                Request($p<Request>),
                Route($p<Route>),
                WebSocket($p<WebSocket>),
                Worker($p<Worker>),
                Dialog($p<Dialog>),
                Download($p<Download>),
                ConsoleMessage($p<ConsoleMessage>),
                JsHandle($p<JsHandle>),
                ElementHandle($p<ElementHandle>),
                Playwright($p<Playwright>)
            }
        };
    }

    remote_enum! {RemoteArc, Arc}

    remote_enum! {RemoteWeak, Weak}

    macro_rules! downgrade {
        ($($t:ident),*) => {
            pub(crate) fn downgrade(&self) -> RemoteWeak {
                match self {
                    $(
                        Self::$t(x) => RemoteWeak::$t(Arc::downgrade(x))
                    ),*
                }
            }
        }
    }

    macro_rules! handle_event {
        ($($t:ident),*) => {
            pub(crate) fn handle_event(&self, ctx: &Context, method: &S<Method>, params: &Map<String, Value>) -> Result<(), Error> {
                match self {
                    $(
                        Self::$t(x) => x.handle_event(ctx, method, params)
                    ),*
                }
            }
        }
    }

    macro_rules! channel {
        ($($t:ident),*) => {
            pub(crate) fn channel(&self) -> &ChannelOwner {
                match self {
                    $(
                        Self::$t(x) => x.channel()
                    ),*
                }
            }
        }
    }

    macro_rules! methods {
        ($($t:ident),*) => {
            downgrade!{$($t),*}
            handle_event!{$($t),*}
            channel!{$($t),*}
        }
    }

    macro_rules! upgrade {
        ($($t:ident),*) => {
            pub(crate) fn upgrade(&self) -> Option<RemoteArc> {
                match self {
                    $(
                        Self::$t(x) => x.upgrade().map(RemoteArc::$t)
                    ),*
                }
            }
        }
    }

    impl RemoteWeak {
        upgrade! {
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
            JsHandle,
            ElementHandle,
            Playwright
        }
    }

    impl RemoteArc {
        methods! {
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
            JsHandle,
            ElementHandle,
            Playwright
        }

        pub(crate) fn try_new(
            typ: &S<ObjectType>,
            ctx: &Context,
            c: ChannelOwner
        ) -> Result<RemoteArc, Error> {
            let r = match typ.as_str() {
                "Playwright" => RemoteArc::Playwright(Arc::new(Playwright::try_new(ctx, c)?)),
                "Selectors" => RemoteArc::Selectors(Arc::new(Selectors::new(c))),
                "BrowserType" => RemoteArc::BrowserType(Arc::new(BrowserType::try_new(c)?)),
                "Browser" => RemoteArc::Browser(Arc::new(Browser::try_new(c)?)),
                "BrowserContext" => {
                    RemoteArc::BrowserContext(Arc::new(BrowserContext::try_new(c)?))
                }
                "Page" => RemoteArc::Page(Arc::new(Page::try_new(ctx, c)?)),
                "Frame" => RemoteArc::Frame(Arc::new(Frame::new(c))),
                "Response" => RemoteArc::Response(Arc::new(Response::try_new(ctx, c)?)),
                "Request" => RemoteArc::Request(Arc::new(Request::try_new(ctx, c)?)),
                "Route" => RemoteArc::Route(Arc::new(Route::try_new(ctx, c)?)),
                "WebSocket" => RemoteArc::WebSocket(Arc::new(WebSocket::try_new(ctx, c)?)),
                "Worker" => RemoteArc::Worker(Arc::new(Worker::new(c))),
                "Dialog" => RemoteArc::Dialog(Arc::new(Dialog::new(c))),
                "Download" => RemoteArc::Download(Arc::new(Download::new(c))),
                "ConsoleMessage" => RemoteArc::ConsoleMessage(Arc::new(ConsoleMessage::new(c))),
                "JsHandle" => RemoteArc::JsHandle(Arc::new(JsHandle::try_new(ctx, c)?)),
                "ElementHandle" => RemoteArc::ElementHandle(Arc::new(ElementHandle::new(c))),
                _ => RemoteArc::Dummy(Arc::new(DummyObject::new(c)))
            };
            Ok(r)
        }
    }
}

pub(crate) use remote_enum::{RemoteArc, RemoteWeak};

pub(crate) struct RequestBody {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>,
    pub(crate) place: WaitPlaces<WaitMessageResult>
}

impl RequestBody {
    pub(crate) fn new(guid: Str<Guid>, method: Str<Method>) -> RequestBody {
        RequestBody {
            guid,
            method,
            params: Map::default(),
            place: WaitPlaces::new_empty()
        }
    }

    // pub(crate) fn set_method(mut self, method: Str<Method>) -> Self {
    //    self.method = method;
    //    self
    //}

    pub(crate) fn set_params(mut self, params: Map<String, Value>) -> Self {
        self.params = params;
        self
    }

    pub(crate) fn set_args<T: Serialize>(self, body: T) -> Result<Self, Error> {
        let v = serde_json::value::to_value(body).map_err(Error::Serde)?;
        let p = match v {
            Value::Object(m) => m,
            _ => return Err(Error::NotObject)
        };
        log::debug!("set request {:?}", &p);
        Ok(self.set_params(p))
    }

    pub(crate) fn set_wait(mut self, wait: &WaitData<WaitMessageResult>) -> Self {
        self.place = wait.place();
        self
    }

    // pub(crate) fn set_guid(mut self, guid: Str<Guid>) -> Self {
    //    self.guid = guid;
    //    self
    //}
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
