use crate::imp::{core::*, impl_future::*, prelude::*};
use serde_json::value::Value;
use std::{fmt::Debug, future::Future, ops::DerefMut, pin::Pin, task::Waker};

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

    pub(crate) fn new_root(ctx: Weak<Mutex<Context>>) -> Self {
        Self {
            ctx,
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
        ctx.lock().send_message(r)?;
        Ok(wait)
    }

    pub(crate) fn children(&self) -> Vec<RemoteWeak> { self.children.lock().to_vec() }

    pub(crate) fn push_child(&self, c: RemoteWeak) {
        let children = &mut self.children.lock();
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
    pub(crate) fn new(ctx: Weak<Mutex<Context>>) -> Self {
        Self {
            channel: ChannelOwner::new_root(ctx)
        }
    }
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
        _method: Str<Method>,
        _params: Map<String, Value>
    ) -> Result<(), Error> {
        Ok(())
    }
}

mod remote_enum {
    use super::{DummyObject as Dummy, RootObject as Root, *};
    use crate::imp::{
        artifact::Artifact, binding_call::BindingCall, browser::Browser,
        browser_context::BrowserContext, browser_type::BrowserType,
        console_message::ConsoleMessage, dialog::Dialog, element_handle::ElementHandle,
        frame::Frame, js_handle::JsHandle, page::Page, playwright::Playwright, request::Request,
        response::Response, route::Route, selectors::Selectors, stream::Stream,
        websocket::WebSocket, worker::Worker
    };

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
            pub(crate) fn handle_event(
                &self, ctx: &Context, method: Str<Method>, params: Map<String, Value>
            ) -> Result<(), Error> {
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

    macro_rules! remote_enum {
        ($($t:ident),*) => {
            #[derive(Debug, Clone)]
            pub(crate) enum RemoteArc {
                $($t(Arc<$t>)),*
            }

            #[derive(Debug, Clone)]
            pub(crate) enum RemoteWeak {
                $($t(Weak<$t>)),*
            }

            impl RemoteWeak {
                upgrade!{$($t),*}
            }

            impl RemoteArc {
                downgrade!{$($t),*}
                handle_event!{$($t),*}
                channel!{$($t),*}
            }
        }
    }

    remote_enum! {
        Dummy,
        Root,
        // Android
        // AndroidSocket
        // AndroidDevice
        Artifact,
        BindingCall,
        Browser,
        BrowserContext,
        BrowserType,
        // CdpSession
        ConsoleMessage,
        Dialog,
        // Electron
        // ElectronApplication
        ElementHandle,
        Frame,
        JsHandle,
        Page,
        Playwright,
        Request,
        Response,
        Route,
        Stream,
        Selectors,
        WebSocket,
        Worker
    }

    impl RemoteArc {
        pub(crate) fn try_new(
            typ: &S<ObjectType>,
            ctx: &Context,
            c: ChannelOwner
        ) -> Result<RemoteArc, Error> {
            let typ_as_str = typ.as_str();
            let r = match typ_as_str {
                "Artifact" => RemoteArc::Artifact(Arc::new(Artifact::try_new(c)?)),
                "BindingCall" => RemoteArc::BindingCall(Arc::new(BindingCall::new(c))),
                "Browser" => RemoteArc::Browser(Arc::new(Browser::try_new(c)?)),
                "BrowserContext" => {
                    RemoteArc::BrowserContext(Arc::new(BrowserContext::try_new(c)?))
                }
                "BrowserType" => RemoteArc::BrowserType(Arc::new(BrowserType::try_new(c)?)),
                "ConsoleMessage" => {
                    RemoteArc::ConsoleMessage(Arc::new(ConsoleMessage::try_new(ctx, c)?))
                }
                "Dialog" => RemoteArc::Dialog(Arc::new(Dialog::new(c))),
                "ElementHandle" => RemoteArc::ElementHandle(Arc::new(ElementHandle::new(c))),
                "Frame" => RemoteArc::Frame(Arc::new(Frame::try_new(ctx, c)?)),
                "JSHandle" => RemoteArc::JsHandle(Arc::new(JsHandle::try_new(c)?)),
                "Page" => RemoteArc::Page(Arc::new(Page::try_new(ctx, c)?)),
                "Playwright" => RemoteArc::Playwright(Arc::new(Playwright::try_new(ctx, c)?)),
                "Request" => RemoteArc::Request(Request::try_new(ctx, c)?),
                "Response" => RemoteArc::Response(Arc::new(Response::try_new(ctx, c)?)),
                "Route" => RemoteArc::Route(Arc::new(Route::try_new(ctx, c)?)),
                "Stream" => RemoteArc::Stream(Arc::new(Stream::new(c))),
                "Selectors" => RemoteArc::Selectors(Arc::new(Selectors::new(c))),
                "WebSocket" => RemoteArc::WebSocket(Arc::new(WebSocket::try_new(c)?)),
                "Worker" => RemoteArc::Worker(Arc::new(Worker::try_new(c)?)),
                _ => RemoteArc::Dummy(Arc::new(DummyObject::new(c)))
            };
            Ok(r)
        }
    }
}

use crate::protocol::generated::MetadataLocation;
pub(crate) use remote_enum::{RemoteArc, RemoteWeak};

pub(crate) struct RequestBody {
    pub(crate) guid: Str<Guid>,
    pub(crate) method: Str<Method>,
    pub(crate) params: Map<String, Value>,
    pub(crate) metadata: crate::protocol::generated::Metadata,
    pub(crate) place: WaitPlaces<WaitMessageResult>
}

impl RequestBody {
    pub(crate) fn new(guid: Str<Guid>, method: Str<Method>) -> RequestBody {
        let mut metadata: crate::protocol::generated::Metadata = Default::default();
        metadata.api_name = Some("".into());

        metadata.location = Some(MetadataLocation {
            column: Some(0.into()),
            file: "".to_string(),
            line: Some(0.into())
        });
        RequestBody {
            guid,
            method,
            params: Map::default(),
            metadata,
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
            match this.place.try_lock() {
                None => {
                    pending!()
                }
                Some(mut x) => {
                    let t = x.deref_mut();
                    if let Some(x) = &*t {
                        return Poll::Ready(x.clone());
                    }
                }
            }
        }
        {
            match this.waker.try_lock() {
                None => {
                    pending!()
                }
                Some(mut t) => {
                    let x = t.deref_mut();
                    if x.is_none() {
                        *x = Some(cx.waker().clone());
                    }
                }
            }
        }
        Poll::Pending
    }
}
