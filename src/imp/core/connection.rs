use crate::imp::{self, core::*, playwright::Playwright, prelude::*};
use futures::{
    channel::{mpsc, mpsc::TrySendError},
    stream::{Stream, StreamExt},
    task::{Context, Poll}
};
use std::{
    collections::VecDeque, future::Future, io, path::Path, pin::Pin, process::Stdio,
    sync::TryLockError, thread
};
use tokio::process::{Child, Command};

// 値を待つfutureのHashMapと
pub(crate) struct Connection {
    _child: Child,
    pub(crate) transport: Transport,
    // buf: Vec<message::Response>
    objects: HashMap<Str<Guid>, RemoteRc>,
    tx: UnboundedSender<RequestBody>,
    rx: UnboundedReceiver<RequestBody>,
    conn: Rweak<Mutex<Connection>>,
    que: VecDeque<RequestBody>,
    id: i32,
    callbacks: HashMap<
        i32,
        (
            Rweak<Mutex<Option<WaitMessageResult>>>,
            Rweak<Mutex<Option<Waker>>>
        )
    >
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error("Failed to initialize")]
    InitializationError,
    #[error("Disconnected")]
    ReceiverClosed,
    #[error("Invalid message")]
    InvalidParams,
    #[error("Parent object not found")]
    ParentNotFound,
    #[error("Object not found")]
    ObjectNotFound,
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Failed to send")]
    Channel,
    #[error(transparent)]
    Send(#[from] SendError)
}

impl Connection {
    pub(crate) async fn try_new(exec: &Path) -> io::Result<Rc<Mutex<Connection>>> {
        let mut child = Command::new(exec)
            .args(&["run-driver"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        // TODO: env "NODE_OPTIONS"
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let transport = Transport::try_new(stdin, stdout);
        let objects = {
            let mut d = HashMap::new();
            let root = RootObject::new();
            d.insert(root.guid().to_owned(), RemoteRc::Root(Rc::new(root)));
            d
        };
        let (tx, rx) = mpsc::unbounded();
        let conn = Rc::new(Mutex::new(Connection {
            _child: child,
            transport,
            objects,
            tx,
            rx,
            conn: Rweak::new(),
            que: VecDeque::new(),
            id: 0,
            callbacks: HashMap::new()
        }));
        conn.lock().unwrap().conn = Rc::downgrade(&conn);
        // let c = Arc::downgrade(&conn);
        // tokio::spawn(async move {
        //    loop {
        //        let c = match c.upgrade() {
        //            Some(c) => c,
        //            None => break
        //        };
        //        c.lock().unwrap().next().await;
        //    }
        //});
        Ok(conn)
    }

    pub(crate) fn enqueue(&mut self, r: RequestBody) { self.que.push_back(r); }

    pub(crate) async fn send_message(&mut self, r: RequestBody) -> Result<(), ConnectionError> {
        self.id += 1;
        let RequestBody {
            guid,
            method,
            params,
            place,
            waker
        } = r;
        self.callbacks.insert(self.id, (place, waker));
        let req = Request {
            guid: &guid,
            method: &method,
            params,
            id: self.id
        };
        self.transport.send(&req).await?;
        Ok(())
    }

    pub(crate) fn get_object(&self, k: &S<Guid>) -> Option<RemoteWeak> {
        self.objects.get(k).map(|r| r.downgrade())
    }

    pub(crate) fn wait_initial_object(c: Rweak<Mutex<Connection>>) -> WaitInitialObject {
        WaitInitialObject(c)
    }

    // async fn processOneMessage(&mut self) -> Result<(), ConnectionError> {
    //    let task = self.transport.next();
    //    let msg = match task.await {
    //        None => return Err(ConnectionError::ReceiverClosed),
    //        Some(msg) => msg
    //    };
    //    self.dispatch(msg).await?;
    //    Ok(())
    //}

    // pub(crate) async fn receive_initializer_message(&mut self) {

    // let guid = "Playwright";

    // if guid in self._objects:
    //    return self._objects[guid]
    // callback = self._loop.create_future()

    // def callback_wrapper(result: Any) -> None:
    //    callback.set_result(result)

    // self._waiting_for_object[guid] = callback_wrapper
    // return await callback
    //}

    fn dispatch(&mut self, msg: Response) -> Result<(), ConnectionError> {
        log::trace!("{:?}", msg);
        match msg {
            Response::Result(msg) => {
                let id = &msg.id;
            }
            Response::Initial(msg) => {
                if Method::is_create(&msg.method) {
                    self.create_remote_object(&msg.guid, msg.params)?;
                    return Ok(());
                }
                if Method::is_dispose(&msg.method) {
                    self.objects.remove(&msg.guid);
                    return Ok(());
                }
                // object.channel.Emit(method, c.replaceGuidsWithChannels(msg.Params))
            }
        }
        Ok(())
    }

    fn create_remote_object(
        &mut self,
        parent: &S<Guid>,
        params: Map<String, Value>
    ) -> Result<(), ConnectionError> {
        let CreateParams {
            typ,
            guid,
            initializer
        } = serde_json::from_value(params.into())?;
        let parent = self
            .objects
            .get(parent)
            .ok_or(ConnectionError::ParentNotFound)?;
        let c = ChannelOwner::new(
            self.conn.clone(),
            self.tx.clone(),
            parent.downgrade(),
            typ.to_owned(),
            guid.to_owned(),
            initializer.to_owned()
        );
        let r = match typ.as_str() {
            "Playwright" => RemoteRc::Playwright(Rc::new(Playwright::try_new(&self, c)?)),
            "Selectors" => RemoteRc::Selectors(Rc::new(imp::selectors::Selectors::new(c))),
            "BrowserType" => {
                RemoteRc::BrowserType(Rc::new(imp::browser_type::BrowserType::try_new(c)?))
            }
            _ => RemoteRc::Dummy(Rc::new(DummyObject::new(c)))
        };
        self.objects.insert(guid.to_owned(), r);
        //(&**parent).push_child(r.clone());
        Ok(())
    }
}

// TODO: レスポンスをさばく, channel読み出してコールバック登録とsend
impl Stream for Connection {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        let this = self.get_mut();
        match Pin::new(&mut this.transport).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(x)) => match this.dispatch(x) {
                Err(e) => {
                    log::error!("{}", e);
                    Poll::Ready(None)
                }
                Ok(_) => Poll::Ready(Some(()))
            }
        }
    }
}

pub(crate) struct WaitInitialObject(Rweak<Mutex<Connection>>);

impl Future for WaitInitialObject {
    type Output = Result<Rweak<Playwright>, ConnectionError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let i: &S<Guid> = S::validate("Playwright").unwrap();
        // FIXME: timeout
        let this = self.get_mut();
        let rc = this.0.upgrade().ok_or(ConnectionError::ObjectNotFound)?;
        let mut c = match rc.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => return Poll::Pending,
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

#[cfg(test)]
mod tests {
    use crate::imp::core::*;
    use std::env;

    crate::runtime_test!(try_new, {
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let _conn = driver.run().await.unwrap();
    });
}
