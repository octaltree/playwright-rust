use crate::imp::{core::*, prelude::*};
use std::{
    io,
    process::{Child, Command, Stdio},
    sync::TryLockError
};
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Event {}

#[derive(Debug)]
pub(crate) struct Context {
    evt_tx: Option<broadcast::Sender<Arc<Event>>>,
    objects: HashMap<Str<Guid>, RemoteArc>,
    ctx: Wm<Context>,
    id: i32,
    #[allow(clippy::type_complexity)]
    callbacks: HashMap<i32, WaitPlaces<WaitMessageResult>>,
    writer: Writer
}

#[derive(Debug)]
pub(crate) struct Connection {
    _child: Child,
    ctx: Am<Context>,
    reader: Am<Reader>,
    stopped: Am<bool>
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
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
    ErrorResponded(#[from] Arc<ErrorMessage>),
    #[error("Value is not Object")]
    NotObject,
    #[error("guid not found in {0:?}")]
    GuidNotFound(Value)
}

pub(crate) fn only_guid(v: &Value) -> Result<&S<Guid>, Error> {
    as_only_guid(v).ok_or(Error::GuidNotFound(v.clone()))
}

impl Drop for Connection {
    fn drop(&mut self) { *self.stopped.lock().unwrap() = true; }
}

impl Connection {
    fn try_new(exec: &Path) -> io::Result<Connection> {
        let mut child = Command::new(exec)
            .args(&["run-driver"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            //.stderr(Stdio::null())
            .spawn()?;
        // TODO: env "NODE_OPTIONS"
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let reader = Reader::new(stdout);
        let writer = Writer::new(stdin);
        let ctx = Context::new(writer);
        Ok(Self {
            _child: child,
            ctx,
            stopped: Arc::new(Mutex::new(false)),
            reader: Arc::new(Mutex::new(reader))
        })
    }

    pub(crate) fn run(exec: &Path) -> io::Result<Connection> {
        let conn = Self::try_new(exec)?;
        conn.start();
        Ok(conn)
    }

    fn start(&self) {
        let c2 = Arc::downgrade(&self.ctx);
        let r2 = Arc::downgrade(&self.reader);
        let s2 = Arc::downgrade(&self.stopped);
        std::thread::spawn(move || {
            log::trace!("succcess starting connection");
            let c = c2;
            let r = r2;
            let s = s2;
            loop {
                let response = {
                    let r = match r.upgrade() {
                        Some(x) => x,
                        None => break
                    };
                    let mut reader = match r.try_lock() {
                        Ok(x) => x,
                        Err(TryLockError::WouldBlock) => continue,
                        Err(e) => Err(e).unwrap()
                    };
                    match reader.try_read() {
                        Ok(Some(x)) => x,
                        Ok(None) => continue,
                        Err(e) => Err(e).unwrap()
                    }
                };
                {
                    let s = match s.upgrade() {
                        Some(x) => x,
                        None => break
                    };
                    let stopped = match s.try_lock() {
                        Ok(x) => *x,
                        Err(TryLockError::WouldBlock) => continue,
                        Err(e) => Err(e).unwrap()
                    };
                    if stopped {
                        break;
                    }
                }
                // dispatch
                {
                    let c = match c.upgrade() {
                        Some(x) => x,
                        None => break
                    };
                    let mut ctx = match c.lock() {
                        Ok(x) => x,
                        Err(e) => Err(e).unwrap()
                    };
                    ctx.dispatch(response).unwrap()
                }
            }
            log::trace!("Done");
        });
    }

    pub(crate) fn context(&self) -> Wm<Context> { Arc::downgrade(&self.ctx) }
}

impl Context {
    fn new(writer: Writer) -> Am<Context> {
        let objects = {
            let mut d = HashMap::new();
            let root = RootObject::new();
            d.insert(root.guid().to_owned(), RemoteArc::Root(Arc::new(root)));
            d
        };
        let ctx = Context {
            evt_tx: None,
            objects,
            ctx: Weak::new(),
            id: 0,
            callbacks: HashMap::new(),
            writer
        };
        let am = Arc::new(Mutex::new(ctx));
        am.lock().unwrap().ctx = Arc::downgrade(&am);
        am
    }

    fn dispatch(&mut self, msg: Res) -> Result<(), Error> {
        match msg {
            Res::Result(msg) => {
                let WaitPlaces { value, waker } =
                    self.callbacks.get(&msg.id).ok_or(Error::CallbackNotFound)?;
                let place = match value.upgrade() {
                    Some(p) => p,
                    None => return Ok(())
                };
                let waker = match waker.upgrade() {
                    Some(x) => x,
                    None => return Ok(())
                };
                *place.lock().unwrap() = Some(Ok(msg.body.map(Arc::new).map_err(Arc::new)));
                let waker: &Option<Waker> = &waker.lock().unwrap();
                let waker = match waker {
                    Some(x) => x.clone(),
                    None => return Ok(())
                };
                waker.wake();
                return Ok(());
            }
            Res::Initial(msg) => {
                if Method::is_create(&msg.method) {
                    self.create_remote_object(&msg.guid, msg.params)?;
                    //(&**parent).push_child(r.clone());
                    return Ok(());
                }
                if Method::is_dispose(&msg.method) {
                    self.objects.remove(&msg.guid);
                    // TODO: dispose children and notify parent
                    return Ok(());
                }
                // TODO: object.channel.Emit(method, c.replaceGuidsWithChannels(msg.Params))
            }
        }
        Ok(())
    }

    fn create_remote_object(
        &mut self,
        parent: &S<Guid>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        let CreateParams {
            typ,
            guid,
            initializer
        } = serde_json::from_value(params.into())?;
        let parent = self.objects.get(parent).ok_or(Error::ParentNotFound)?;
        let c = ChannelOwner::new(
            self.ctx.clone(),
            parent.downgrade(),
            typ.to_owned(),
            guid.to_owned(),
            initializer
        );
        let r = RemoteArc::try_new(&typ, &self, c)?;
        self.objects.insert(guid, r);
        Ok(())
    }

    pub(in crate::imp) fn get_object(&self, k: &S<Guid>) -> Option<RemoteWeak> {
        self.objects.get(k).map(|r| r.downgrade())
    }

    pub(in crate::imp::core) fn send_message(&mut self, r: RequestBody) -> Result<(), Error> {
        self.id += 1;
        let RequestBody {
            guid,
            method,
            params,
            place
        } = r;
        self.callbacks.insert(self.id, place);
        let req = Req {
            guid: &guid,
            method: &method,
            params,
            id: self.id
        };
        self.writer.send(&req)?;
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
}

#[cfg(test)]
mod tests {
    use crate::imp::{core::*, prelude::*};

    crate::runtime_test!(start, {
        let driver = Driver::install().unwrap();
        let conn = Connection::try_new(&driver.executable()).unwrap();
        Connection::start(&conn);
    });

    crate::runtime_test!(tokio_event, {
        let driver = Driver::install().unwrap();
        let conn = Connection::try_new(&driver.executable()).unwrap();
        Connection::start(&conn);
        let mut rx = conn.ctx.lock().unwrap().subscribe_event();
        conn.ctx.lock().unwrap().emit_event(Event {});
        rx.recv().await.unwrap();
    });
}
