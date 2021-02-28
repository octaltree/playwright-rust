use crate::imp::{
    self, message,
    playwright::Playwright,
    remote_object::{ChannelOwner, DummyObject, RemoteObject},
    transport::Transport
};
use futures::{
    stream::{Stream, StreamExt},
    task::{Context, Poll}
};
use serde_json::{map::Map, value::Value};
use std::{any::Any, collections::HashMap, io, path::Path, pin::Pin, process::Stdio, sync::Arc};
use strong::*;
use thiserror::Error;
use tokio::process::{Child, Command};

// 値を待つfutureのHashMapと
pub(crate) struct Connection {
    _child: Child,
    pub(crate) transport: Transport,
    objects: HashMap<Str<message::Guid>, Arc<dyn RemoteObject>> /* buf: Vec<message::Response> /* root: Option<Arc<ChannelOwner<'_>>> /* owners: HashMap<Str<message::Guid>, Arc<dyn HasChannelOwner>> */ */ */
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Failed to initialize")]
    InitializationError,
    #[error("Disconnected")]
    ReceiverClosed,
    #[error("Invalid message")]
    InvalidParams,
    #[error("Parent object not found")]
    ParentNotFound
}

impl Connection {
    pub(crate) async fn try_new(exec: &Path) -> io::Result<Connection> {
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
            let root = DummyObject::new_root();
            d.insert(
                root.guid().to_owned(),
                Arc::new(root) as Arc<dyn RemoteObject>
            );
            d
        };
        Ok(Connection {
            _child: child,
            transport,
            objects // buf: Vec::new() // root: None // owners: HashMap::new(),
        })
    }

    pub(crate) async fn wait_initial_object(&mut self) -> Result<Arc<Playwright>, ConnectionError> {
        let guid: &S<message::Guid> = &S::validate("Playwright").unwrap();
        let o = loop {
            if let Some(o) = self.objects.get(guid) {
                break o.clone();
            }
            self.next().await.ok_or(ConnectionError::ReceiverClosed)?;
        };
        let p = o
            .downcast_arc::<Playwright>()
            .map_err(|_| ConnectionError::InitializationError)?;
        return Ok(p);
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

    fn dispatch(&mut self, msg: message::Response) -> Result<(), ConnectionError> {
        log::trace!("{:?}", msg);
        match msg {
            message::Response::Result(msg) => {
                let id = &msg.id;
            }
            message::Response::Initial(msg) => {
                if message::Method::is_create(&msg.method) {
                    self.create_remote_object(&msg.guid, msg.params)?;
                    return Ok(());
                }
                if message::Method::is_dispose(&msg.method) {
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
        parent: &S<message::Guid>,
        params: Map<String, Value>
    ) -> Result<(), ConnectionError> {
        let typ: &S<message::ObjectType> = S::validate(
            params
                .get("type")
                .ok_or(ConnectionError::InvalidParams)?
                .as_str()
                .ok_or(ConnectionError::InvalidParams)?
        )
        .unwrap();
        let guid: &S<message::Guid> = S::validate(
            params
                .get("guid")
                .ok_or(ConnectionError::InvalidParams)?
                .as_str()
                .ok_or(ConnectionError::InvalidParams)?
        )
        .unwrap();
        let parent = self
            .objects
            .get(parent)
            .ok_or(ConnectionError::ParentNotFound)?;
        // match typ.as_str() {
        //    "Playwright" => Playwright(),
        //    _ => DummyObject::new()
        //}
        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use crate::imp::driver::Driver;
    use std::env;

    crate::runtime_test!(try_new, {
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let _conn = driver.run().await.unwrap();
    });
}
