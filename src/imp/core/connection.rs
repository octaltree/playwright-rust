use crate::imp::{core::*, prelude::*};
use futures::{
    stream::{Stream, StreamExt},
    task::{Context, Poll}
};
use std::{
    future::Future,
    io,
    path::Path,
    pin::Pin,
    process::{Child, Command, Stdio},
    sync::TryLockError
};
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Event {}

pub(crate) struct Connection {
    evt_tx: Option<broadcast::Sender<Arc<Event>>>,
    _child: Child,
    transport: Transport,
    objects: HashMap<Str<Guid>, RemoteArc>,
    conn: Wm<Connection>,
    id: i32,
    #[allow(clippy::type_complexity)]
    callbacks: HashMap<i32, (Wm<Option<WaitMessageResult>>, Wm<Option<Waker>>)>,
    stopped: bool
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
    Transport(#[from] TransportError),
    #[error("Callback not found")]
    CallbackNotFound,
    #[error(transparent)]
    ErrorResponded(#[from] Arc<Error>),
    #[error("Value is not Object")]
    NotObject
}

pub(crate) struct Running {
    conn: Weak<Mutex<Connection>>
}

impl Drop for Running {
    fn drop(&mut self) {
        let conn = match self.conn.upgrade() {
            Some(c) => c,
            None => return
        };
        conn.lock().unwrap().stopped = true;
    }
}

impl Connection {
    pub(crate) fn run(exec: &Path) -> io::Result<(Am<Connection>, Running)> {
        let conn = Self::try_new(exec)?;
        let running = Self::start(&conn);
        Ok((conn, running))
    }

    fn try_new(exec: &Path) -> io::Result<Am<Connection>> {
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
            d.insert(root.guid().to_owned(), RemoteArc::Root(Arc::new(root)));
            d
        };
        let conn = Arc::new(Mutex::new(Connection {
            evt_tx: Option::<broadcast::Sender<Arc<Event>>>::default(),
            _child: child,
            transport,
            objects,
            conn: Weak::new(),
            id: 0,
            callbacks: HashMap::new(),
            stopped: false
        }));
        conn.lock().unwrap().conn = Arc::downgrade(&conn);
        Ok(conn)
    }

    pub(in crate::imp) fn weak(&self) -> Wm<Connection> { self.conn.clone() }

    pub(in crate::imp) fn get_object(&self, k: &S<Guid>) -> Option<RemoteWeak> {
        self.objects.get(k).map(|r| r.downgrade())
    }

    fn send_message(&mut self, r: RequestBody) -> Result<(), ConnectionError> {
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
        self.transport.send(&req)?;
        Ok(())
    }

    fn subscribe_event(&mut self) -> broadcast::Receiver<Arc<Event>> {
        if let Some(tx) = &self.evt_tx {
            return tx.subscribe();
        }
        let (tx, rx) = broadcast::channel(100);
        self.evt_tx = Some(tx);
        return rx;
    }

    fn emit_event<E: Into<Arc<Event>>>(&self, e: E) {
        let tx = match &self.evt_tx {
            None => return,
            Some(tx) => tx
        };
        tx.send(e.into()).ok();
    }

    fn start(this: &Am<Connection>) -> Running {
        let weak = Arc::downgrade(this);
        let running = Running { conn: weak.clone() };
        spawn(async move {
            log::trace!("succcess starting connection");
            let c = weak.clone();
            (|| -> Result<(), ConnectionError> {
                while let Some(rc) = c.upgrade() {
                    let m = &mut match rc.try_lock() {
                        Ok(x) => x,
                        Err(TryLockError::WouldBlock) => continue,
                        Err(e) => Err(e).unwrap()
                    };
                    if m.stopped {
                        break;
                    }
                    let r = m.transport.try_read()?;
                    let r = match r {
                        Some(r) => r,
                        None => continue
                    };
                    m.dispatch(r)?;
                }
                Ok(())
            })()
            .unwrap();
        });
        running
    }

    fn dispatch(&mut self, msg: Response) -> Result<(), ConnectionError> {
        match msg {
            Response::Result(msg) => {
                // let (place, waker) = self
                //    .callbacks
                //    .get(&msg.id)
                //    .ok_or(ConnectionError::CallbackNotFound)?;
                // let place = match place.upgrade() {
                //    Some(p) => p,
                //    None => return Ok(())
                //};
                // let waker = match waker.upgrade() {
                //    Some(x) => x,
                //    None => return Ok(())
                //};
                // log::trace!("success get rc");
                //*place.lock().unwrap() = Some(Ok(msg.body.map(Arc::new).map_err(Arc::new)));
                // let waker: &Option<Waker> = &waker.lock().unwrap();
                // let waker = match waker {
                //    Some(x) => x.clone(),
                //    None => return Ok(())
                //};
                // waker.wake();
                return Ok(());
            }
            Response::Initial(msg) => {
                if Method::is_create(&msg.method) {
                    self.create_remote_object(&msg.guid, msg.params)?;
                    return Ok(());
                }
                // if Method::is_dispose(&msg.method) {
                //    self.objects.remove(&msg.guid);
                //    return Ok(());
                //}
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
            // self.tx.clone(),
            parent.downgrade(),
            typ.to_owned(),
            guid.to_owned(),
            initializer
        );
        let r = RemoteArc::try_new(&typ, &self, c)?;
        self.objects.insert(guid, r);
        //(&**parent).push_child(r.clone());
        Ok(())
    }
}

// impl Connection {

//    // pub(crate) fn wait_initial_object(c: Weak<Mutex<Connection>>) -> WaitInitialObject {
//    //    WaitInitialObject(c)
//    //}

//    // async fn processOneMessage(&mut self) -> Result<(), ConnectionError> {
//    //    let task = self.transport.next();
//    //    let msg = match task.await {
//    //        None => return Err(ConnectionError::ReceiverClosed),
//    //        Some(msg) => msg
//    //    };
//    //    self.dispatch(msg).await?;
//    //    Ok(())
//    //}

//    // pub(crate) async fn receive_initializer_message(&mut self) {

//    // let guid = "Playwright";

//    // if guid in self._objects:
//    //    return self._objects[guid]
//    // callback = self._loop.create_future()

//    // def callback_wrapper(result: Any) -> None:
//    //    callback.set_result(result)

//    // self._waiting_for_object[guid] = callback_wrapper
//    // return await callback
//    //}

//    fn dispatch(&mut self, msg: Response) -> Result<(), ConnectionError> {
//        log::trace!("{:?}", msg);
//        match msg {
//            Response::Result(msg) => {
//                log::trace!("result");
//                let (place, waker) = self
//                    .callbacks
//                    .get(&msg.id)
//                    .ok_or(ConnectionError::CallbackNotFound)?;
//                let place = match place.upgrade() {
//                    Some(p) => p,
//                    None => return Ok(())
//                };
//                let waker = match waker.upgrade() {
//                    Some(x) => x,
//                    None => return Ok(())
//                };
//                log::trace!("success get rc");
//                *place.lock().unwrap() = Some(Ok(msg.body.map(Arc::new).map_err(Arc::new)));
//                let waker: &Option<Waker> = &waker.lock().unwrap();
//                let waker = match waker {
//                    Some(x) => x.clone(),
//                    None => return Ok(())
//                };
//                waker.wake();
//                return Ok(());
//            }
//            Response::Initial(msg) => {
//                if Method::is_create(&msg.method) {
//                    self.create_remote_object(&msg.guid, msg.params)?;
//                    return Ok(());
//                }
//                if Method::is_dispose(&msg.method) {
//                    self.objects.remove(&msg.guid);
//                    return Ok(());
//                }
//                // object.channel.Emit(method, c.replaceGuidsWithChannels(msg.Params))
//            }
//        }
//        Ok(())
//    }

//    fn create_remote_object(
//        &mut self,
//        parent: &S<Guid>,
//        params: Map<String, Value>
//    ) -> Result<(), ConnectionError> {
//        let CreateParams {
//            typ,
//            guid,
//            initializer
//        } = serde_json::from_value(params.into())?;
//        let parent = self
//            .objects
//            .get(parent)
//            .ok_or(ConnectionError::ParentNotFound)?;
//        let c = ChannelOwner::new(
//            self.conn.clone(),
//            // self.tx.clone(),
//            parent.downgrade(),
//            typ.to_owned(),
//            guid.to_owned(),
//            initializer
//        );
//        let r = RemoteArc::try_new(&typ, &self, c)?;
//        self.objects.insert(guid, r);
//        //(&**parent).push_child(r.clone());
//        Ok(())
//    }
//}

#[cfg(test)]
mod tests {
    use crate::imp::core::*;

    crate::runtime_test!(start, {
        let driver = Driver::install().unwrap();
        let conn = Connection::try_new(&driver.executable()).unwrap();
        let _running = Connection::start(&conn);
    });
}
