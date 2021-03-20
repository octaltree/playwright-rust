use crate::imp::{core::*, prelude::*};
use std::{
    io,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        TryLockError
    }
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
    callbacks: HashMap<i32, WaitPlaces<WaitMessageResult>>,
    writer: Writer
}

#[derive(Debug)]
pub(crate) struct Connection {
    _child: Child,
    ctx: Am<Context>,
    reader: Am<Reader>,
    should_stop: Arc<AtomicBool>
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
    GuidNotFound(Value),
    #[error(transparent)]
    InvalidBase64(#[from] base64::DecodeError),
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    SerializationPwJson(#[from] ser::Error),
    #[error(transparent)]
    DeserializationPwJson(#[from] de::Error),
    #[error(transparent)]
    Arc(#[from] Arc<Error>)
}

pub(crate) type ArcResult<T> = Result<T, Arc<Error>>;

impl Drop for Connection {
    fn drop(&mut self) {
        self.notify_closed();
        self.should_stop.store(true, Ordering::Relaxed);
    }
}

impl Connection {
    fn try_new(exec: &Path) -> io::Result<Connection> {
        let mut child = Command::new(exec)
            .args(&["run-driver"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
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
            should_stop: Arc::new(false.into()),
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
        let s2 = Arc::downgrade(&self.should_stop);
        std::thread::spawn(move || {
            log::trace!("succcess starting connection");
            let c = c2;
            let r = r2;
            let s = s2;
            let status = (|| -> Result<(), Error> {
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
                        match reader.try_read()? {
                            Some(x) => x,
                            None => continue
                        }
                    };
                    {
                        let s = match s.upgrade() {
                            Some(x) => x,
                            None => break
                        };
                        let should_stop = s.load(Ordering::Relaxed);
                        if should_stop {
                            break;
                        }
                    }
                    // dispatch
                    {
                        let c = match c.upgrade() {
                            Some(x) => x,
                            None => break
                        };
                        let mut ctx = c.lock().unwrap();
                        ctx.dispatch(response)?;
                        log::debug!("{:?}", ctx.objects.keys());
                    }
                }
                Ok(())
            })();
            if status.is_ok() {
                log::trace!("Done");
            } else {
                log::trace!("Failed");
                if let Some(c) = c.upgrade() {
                    let mut ctx = c.lock().unwrap();
                    ctx.notify_closed();
                }
            }
        });
    }

    pub(crate) fn context(&self) -> Wm<Context> { Arc::downgrade(&self.ctx) }

    fn notify_closed(&mut self) {
        let ctx = &mut self.ctx.lock().unwrap();
        ctx.notify_closed();
    }
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

    fn notify_closed(&mut self) {
        for p in self.callbacks.iter().map(|(_, v)| v) {
            Context::respond_wait(p, Err(Arc::new(Error::ReceiverClosed)));
        }
        self.objects = HashMap::new();
    }

    fn dispatch(&mut self, msg: Res) -> Result<(), Error> {
        match msg {
            Res::Result(msg) => {
                let p = self.callbacks.get(&msg.id).ok_or(Error::CallbackNotFound)?;
                Self::respond_wait(p, Ok(msg.body.map(Arc::new).map_err(Arc::new)));
                return Ok(());
            }
            Res::Initial(msg) => {
                if Method::is_create(&msg.method) {
                    self.create_remote_object(&msg.guid, msg.params)?;
                    //(&**parent).push_child(r.clone());
                    return Ok(());
                }
                if Method::is_dispose(&msg.method) {
                    self.dispose(&msg.guid);
                    return Ok(());
                }
                let target = self.objects.get(&msg.guid).ok_or(Error::ObjectNotFound)?;
                target.handle_event(self, &msg.method, &msg.params)?;
            }
        }
        Ok(())
    }

    fn dispose(&mut self, i: &S<Guid>) {
        let a = match self.objects.get(i) {
            None => return,
            Some(a) => a
        };
        let cs = a.channel().children();
        for c in cs {
            let c = match c.upgrade() {
                None => continue,
                Some(c) => c
            };
            self.dispose(&c.channel().guid);
        }
        self.remove_object(i);
    }

    fn respond_wait(
        WaitPlaces { value, waker }: &WaitPlaces<WaitMessageResult>,
        result: WaitMessageResult
    ) {
        let place = match value.upgrade() {
            Some(p) => p,
            None => return
        };
        let waker = match waker.upgrade() {
            Some(x) => x,
            None => return
        };
        *place.lock().unwrap() = Some(result);
        let waker: &Option<Waker> = &waker.lock().unwrap();
        let waker = match waker {
            Some(x) => x.clone(),
            None => return
        };
        waker.wake();
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
        let parent = self.objects.get(parent).ok_or(Error::ObjectNotFound)?;
        let c = ChannelOwner::new(
            self.ctx.clone(),
            parent.downgrade(),
            typ.to_owned(),
            guid.to_owned(),
            initializer
        );
        let r = RemoteArc::try_new(&typ, &self, c)?;
        parent.channel().push_child(r.downgrade());
        self.objects.insert(guid, r.clone());
        if let RemoteArc::Page(p) = r.clone() {
            p.hook_created(Arc::downgrade(&p))?;
        }
        Ok(())
    }

    pub(in crate::imp) fn find_object(&self, k: &S<Guid>) -> Option<RemoteWeak> {
        self.objects.get(k).map(|r| r.downgrade())
    }

    pub(in crate::imp) fn remove_object(&mut self, k: &S<Guid>) { self.objects.remove(k); }

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
        rx
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
    use crate::imp::core::*;

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
