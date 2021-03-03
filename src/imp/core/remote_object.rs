use crate::imp::{self, core::*, prelude::*};
use futures::{
    channel::mpsc,
    stream::Stream,
    task::{Context, Poll}
};
use serde_json::value::Value;
use std::{
    any::Any,
    fmt::{self, Debug},
    future::Future,
    pin::Pin,
    sync::TryLockError,
    task::Waker
};

pub(crate) struct ChannelOwner {
    pub(crate) conn: Rweak<Mutex<Connection>>,
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
        conn: Rweak<Mutex<Connection>>,
        // tx: UnboundedSender<RequestBody>,
        parent: RemoteWeak,
        typ: Str<ObjectType>,
        guid: Str<Guid>,
        initializer: Value
    ) -> Self {
        Self {
            conn,
            parent: Some(parent),
            typ,
            guid,
            initializer
        }
    }

    pub(crate) fn new_root() -> Self {
        // let (tx, _) = mpsc::unbounded();
        Self {
            conn: Rweak::new(),
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
            place: Rweak::new(),
            waker: Rweak::new()
        }
    }

    pub(crate) async fn send_message(
        &self,
        r: RequestBody
    ) -> Result<WaitMessage, ConnectionError> {
        let w = WaitMessage::new(self.conn.clone());
        let r = r.set_wait(&w);
        // self.tx
        //    .unbounded_send(r)
        //    .map_err(|_| ConnectionError::Channel)?;
        let conn = self.conn.upgrade().ok_or(ConnectionError::ObjectNotFound)?;
        conn.lock().unwrap().send_message(r).await?;
        Ok(w)
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
}

#[derive(Debug)]
pub(crate) enum RemoteRc {
    Dummy(Rc<DummyObject>),
    Root(Rc<RootObject>),
    Playwright(Rc<imp::playwright::Playwright>),
    BrowserType(Rc<imp::browser_type::BrowserType>),
    Selectors(Rc<imp::selectors::Selectors>)
}

#[derive(Debug)]
pub(crate) enum RemoteWeak {
    Dummy(Rweak<DummyObject>),
    Root(Rweak<RootObject>),
    Playwright(Rweak<imp::playwright::Playwright>),
    BrowserType(Rweak<imp::browser_type::BrowserType>),
    Selectors(Rweak<imp::selectors::Selectors>)
}

impl RemoteRc {
    pub(crate) fn downgrade(&self) -> RemoteWeak {
        match self {
            Self::Dummy(x) => RemoteWeak::Dummy(Rc::downgrade(x)),
            Self::Root(x) => RemoteWeak::Root(Rc::downgrade(x)),
            Self::Playwright(x) => RemoteWeak::Playwright(Rc::downgrade(x)),
            Self::BrowserType(x) => RemoteWeak::BrowserType(Rc::downgrade(x)),
            Self::Selectors(x) => RemoteWeak::Selectors(Rc::downgrade(x))
        }
    }
}

pub(crate) struct RequestBody {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>,
    pub(crate) place: Rweak<Mutex<Option<WaitMessageResult>>>,
    pub(crate) waker: Rweak<Mutex<Option<Waker>>>
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
        self.place = Rc::downgrade(&wait.place);
        self.waker = Rc::downgrade(&wait.waker);
        self
    }

    pub(crate) fn set_guid(mut self, guid: Str<Guid>) -> Self {
        self.guid = guid;
        self
    }
}

pub(crate) type WaitMessageResult = Result<Result<Rc<Value>, Rc<Error>>, Rc<ConnectionError>>;

pub(crate) struct WaitMessage {
    place: Rc<Mutex<Option<WaitMessageResult>>>,
    waker: Rc<Mutex<Option<Waker>>>,
    conn: Rweak<Mutex<Connection>>
}

impl WaitMessage {
    pub(crate) fn new(conn: Rweak<Mutex<Connection>>) -> Self {
        let place = Rc::new(Mutex::new(None));
        let waker = Rc::new(Mutex::new(None));
        WaitMessage { place, waker, conn }
    }
}

impl Future for WaitMessage {
    type Output = WaitMessageResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        log::trace!("poll WaitMessage");
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
            // log::trace!("lock success");
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
                log::trace!("set waker");
                *x = Some(cx.waker().clone());
            }
        }

        let rc = this
            .conn
            .upgrade()
            .ok_or_else(|| Rc::new(ConnectionError::ObjectNotFound))?;
        let mut c = match rc.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => pending!(),
            Err(e) => Err(e).unwrap()
        };
        let c: Pin<&mut Connection> = Pin::new(&mut c);
        match c.poll_next(cx) {
            Poll::Ready(None) => Poll::Ready(Err(Rc::new(ConnectionError::ReceiverClosed))),
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
