use crate::imp::{self, core::*, prelude::*};
use futures::{
    channel::mpsc,
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
    pub(crate) tx: UnboundedSender<RequestBody>,
    pub(crate) parent: Option<RemoteWeak>,
    pub(crate) typ: Str<ObjectType>,
    pub(crate) guid: Str<Guid>,
    pub(crate) initializer: Value
}

impl Debug for ChannelOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChannelOwner")
            .field("conn", &"{..}")
            .field("parent", &self.parent)
            .field("typ", &self.typ)
            .field("guid", &self.guid)
            .field("initializer", &self.initializer)
            .finish()
    }
}

impl ChannelOwner {
    pub(crate) fn new(
        tx: UnboundedSender<RequestBody>,
        parent: RemoteWeak,
        typ: Str<ObjectType>,
        guid: Str<Guid>,
        initializer: Value
    ) -> Self {
        Self {
            tx,
            parent: Some(parent),
            typ,
            guid,
            initializer
        }
    }

    pub(crate) fn new_root() -> Self {
        let (tx, _) = mpsc::unbounded();
        Self {
            tx,
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
    Dummy(Weak<DummyObject>),
    Root(Weak<RootObject>),
    Playwright(Weak<imp::playwright::Playwright>),
    BrowserType(Weak<imp::browser_type::BrowserType>),
    Selectors(Weak<imp::selectors::Selectors>)
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
    guid: Str<Guid>,
    method: Str<Method>,
    params: Map<String, Value>,
    place: Weak<Mutex<Option<Rc<ResponseResult>>>>,
    waker: Weak<Mutex<Option<Waker>>>
}

impl RequestBody {
    pub(crate) fn set_params(mut self, params: Map<String, Value>) -> Self {
        self.params = params;
        self
    }

    pub(crate) fn set_wait(mut self, wait: &WaitMessage) -> Self {
        self.place = Rc::downgrade(&wait.place);
        self.waker = Rc::downgrade(&wait.waker);
        self
    }
}

pub(crate) struct WaitMessage {
    // FIXME: Option<Result<ResponseResult, ConnectionError>>
    place: Rc<Mutex<Option<Rc<ResponseResult>>>>,
    waker: Rc<Mutex<Option<Waker>>>
}

impl WaitMessage {
    pub(crate) fn new() -> Self {
        let place = Rc::new(Mutex::new(None));
        let waker = Rc::new(Mutex::new(None));
        let weak = Rc::downgrade(&place);
        WaitMessage { place, waker }
    }
}

impl Future for WaitMessage {
    type Output = Rc<ResponseResult>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
        let x = match this.place.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => pending!(),
            Err(e) => Err(e).unwrap()
        };
        if let Some(x) = &*x {
            return Poll::Ready(Rc::clone(x));
        } else {
            pending!()
        }
    }
}
