use crate::imp::{connection::Connection, message, prelude::*};
use serde_json::value::Value;
use std::fmt::{self, Debug};

pub(crate) trait RemoteObject: Debug {
    fn channel(&self) -> &ChannelOwner;
    fn channel_mut(&mut self) -> &mut ChannelOwner;

    fn guid(&self) -> &S<message::Guid> { &self.channel().guid }
}

pub(crate) struct ChannelOwner {
    pub(crate) conn: Weak<RefCell<Connection>>,
    pub(crate) parent: Option<Weak<dyn RemoteObject>>,
    pub(crate) typ: Str<message::ObjectType>,
    pub(crate) guid: Str<message::Guid>,
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
        conn: Weak<RefCell<Connection>>,
        parent: Weak<dyn RemoteObject>,
        typ: Str<message::ObjectType>,
        guid: Str<message::Guid>,
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
        Self {
            conn: Weak::new(),
            parent: None,
            typ: Str::validate("".into()).unwrap(),
            guid: Str::validate("".into()).unwrap(),
            initializer: Value::default()
        }
    }
}

#[derive(Debug)]
pub(crate) struct DummyObject {
    channel: ChannelOwner
}

impl DummyObject {
    pub(crate) fn new(channel: ChannelOwner) -> Self { DummyObject { channel } }

    pub(crate) fn new_root() -> Self {
        DummyObject {
            channel: ChannelOwner::new_root()
        }
    }
}

impl RemoteObject for DummyObject {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

// pub(crate) fn create_remote_object(
//    parent: Arc<ChannelOwner>,
//    t: Str<message::ObjectType>,
//    i: Str<message::Guid>,
//    initializer: Value
//) -> Arc<dyn RemoteObject> {
//    let channel = Arc::new(ChannelOwner::new(parent, t, i, initializer));
//    Arc::new(DummyObject::new())
//}
