use crate::imp::{self, core::*, prelude::*};
use serde_json::value::Value;
use std::{
    any::Any,
    fmt::{self, Debug},
    future::Future,
    pin::Pin,
    sync::TryLockError,
    task::Waker
};

pub(crate) fn upgrade<T>(w: &Weak<T>) -> Result<Arc<T>, ConnectionError> {
    w.upgrade().ok_or(ConnectionError::ObjectNotFound)
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

pub(crate) struct ChannelOwner {
    pub(crate) ctx: Weak<Mutex<Context>>,
    // pub(crate) tx: UnboundedSender<RequestBody>,
    pub(crate) parent: Option<RemoteWeak>,
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    pub(crate) initializer: Value
}

impl Debug for ChannelOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChannelOwner")
            .field("parent", &self.parent)
            .field("typ", &self.typ)
            .field("guid", &self.guid)
            .field("initializer", &self.initializer)
            .finish()
    }
}

impl ChannelOwner {
    pub(crate) fn new(
        ctx: Weak<Mutex<Context>>,
        // tx: UnboundedSender<RequestBody>,
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
        // let (tx, _) = mpsc::unbounded();
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
            place: Weak::new(),
            waker: Weak::new()
        }
    }

    // pub(crate) async fn send_message(
    //    &self,
    //    r: RequestBody
    //) -> Result<WaitMessage, ConnectionError> {
    //    let w = WaitMessage::new(self.conn.clone());
    //    let r = r.set_wait(&w);
    //    // self.tx
    //    //    .unbounded_send(r)
    //    //    .map_err(|_| ConnectionError::Channel)?;
    //    let conn = upgrade(&self.conn)?;
    //    conn.lock().unwrap().send_message(r).await?;
    //    Ok(w)
    //}
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
}

#[derive(Debug)]
pub(crate) enum RemoteArc {
    Dummy(Arc<DummyObject>),
    Root(Arc<RootObject>),
    // BrowserType(Arc<imp::browser_type::BrowserType>),
    // Selectors(Arc<imp::selectors::Selectors>),
    // Browser(Arc<imp::browser::Browser>),
    // BrowserContext(Arc<imp::browser_context::BrowserContext>)
    Playwright(Arc<imp::playwright::Playwright>)
}

#[derive(Debug)]
pub(crate) enum RemoteWeak {
    Dummy(Weak<DummyObject>),
    Root(Weak<RootObject>),
    // BrowserType(Weak<imp::browser_type::BrowserType>),
    // Selectors(Weak<imp::selectors::Selectors>),
    // Browser(Weak<imp::browser::Browser>),
    // BrowserContext(Weak<imp::browser_context::BrowserContext>)
    Playwright(Weak<imp::playwright::Playwright>)
}

impl RemoteArc {
    pub(crate) fn downgrade(&self) -> RemoteWeak {
        match self {
            Self::Dummy(x) => RemoteWeak::Dummy(Arc::downgrade(x)),
            Self::Root(x) => RemoteWeak::Root(Arc::downgrade(x)),
            // Self::BrowserType(x) => RemoteWeak::BrowserType(Arc::downgrade(x)),
            // Self::Selectors(x) => RemoteWeak::Selectors(Arc::downgrade(x)),
            // Self::Browser(x) => RemoteWeak::Browser(Arc::downgrade(x)),
            // Self::BrowserContext(x) => RemoteWeak::BrowserContext(Arc::downgrade(x))
            Self::Playwright(x) => RemoteWeak::Playwright(Arc::downgrade(x))
        }
    }

    pub(crate) fn try_new(
        typ: &S<ObjectType>,
        ctx: &Context,
        c: ChannelOwner
    ) -> Result<RemoteArc, ConnectionError> {
        let r = match typ.as_str() {
            "Playwright" => {
                RemoteArc::Playwright(Arc::new(imp::playwright::Playwright::try_new(ctx, c)?))
            }
            //"Selectors" => RemoteArc::Selectors(Arc::new(imp::selectors::Selectors::new(c))),
            //"BrowserType" => {
            //    RemoteArc::BrowserType(Arc::new(imp::browser_type::BrowserType::try_new(c)?))
            //}
            //"Browser" => RemoteArc::Browser(Arc::new(imp::browser::Browser::try_new(c)?)),
            //"BrowserContext" => RemoteArc::BrowserContext(Arc::new(
            //    imp::browser_context::BrowserContext::try_new(c)?
            //)),
            _ => RemoteArc::Dummy(Arc::new(DummyObject::new(c)))
        };
        Ok(r)
    }
}

pub(crate) struct RequestBody {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>,
    pub(crate) place: Weak<Mutex<Option<WaitMessageResult>>>,
    pub(crate) waker: Weak<Mutex<Option<Waker>>>
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

    pub(crate) fn set_args<T: Serialize>(mut self, body: T) -> Result<Self, ConnectionError> {
        let v = serde_json::value::to_value(body).map_err(ConnectionError::Serde)?;
        let p = match v {
            Value::Object(m) => m,
            _ => return Err(ConnectionError::NotObject)
        };
        log::debug!("{:?}", &p);
        self.params = p;
        Ok(self)
    }

    pub(crate) fn set_wait(mut self, wait: &WaitMessage) -> Self {
        self.place = Arc::downgrade(&wait.place);
        self.waker = Arc::downgrade(&wait.waker);
        self
    }

    pub(crate) fn set_guid(mut self, guid: Str<Guid>) -> Self {
        self.guid = guid;
        self
    }
}

pub(crate) type WaitMessageResult = Result<Result<Arc<Value>, Arc<Error>>, Arc<ConnectionError>>;

pub(crate) struct WaitMessage {
    place: Arc<Mutex<Option<WaitMessageResult>>>,
    waker: Arc<Mutex<Option<Waker>>>,
    conn: Weak<Mutex<Connection>>
}

impl WaitMessage {
    pub(crate) fn new(conn: Weak<Mutex<Connection>>) -> Self {
        let place = Arc::new(Mutex::new(None));
        let waker = Arc::new(Mutex::new(None));
        WaitMessage { place, waker, conn }
    }
}

// impl Future for WaitMessage {
//    type Output = WaitMessageResult;

//    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//        let this = self.get_mut();
//        log::trace!("poll WaitMessage");
//        macro_rules! pending {
//            () => {{
//                cx.waker().wake_by_ref();
//                return Poll::Pending;
//            }};
//        }
//        {
//            let x = match this.place.try_lock() {
//                Ok(x) => x,
//                Err(TryLockError::WouldBlock) => pending!(),
//                Err(e) => Err(e).unwrap()
//            };
//            // log::trace!("lock success");
//            if let Some(x) = &*x {
//                return Poll::Ready(x.clone());
//            }
//        }
//        {
//            let mut x = match this.waker.try_lock() {
//                Ok(x) => x,
//                Err(TryLockError::WouldBlock) => pending!(),
//                Err(e) => Err(e).unwrap()
//            };
//            if x.is_none() {
//                log::trace!("set waker");
//                *x = Some(cx.waker().clone());
//            }
//        }

//        let rc = upgrade(&this.conn)?;
//        let mut c = match rc.try_lock() {
//            Ok(x) => x,
//            Err(TryLockError::WouldBlock) => pending!(),
//            Err(e) => Err(e).unwrap()
//        };
//        let c: Pin<&mut Connection> = Pin::new(&mut c);
//        match c.poll_next(cx) {
//            Poll::Ready(None) => Poll::Ready(Err(Arc::new(ConnectionError::ReceiverClosed))),
//            Poll::Pending => {
//                cx.waker().wake_by_ref();
//                Poll::Pending
//            }
//            Poll::Ready(Some(())) => {
//                cx.waker().wake_by_ref();
//                Poll::Pending
//            }
//        }
//    }
//}
