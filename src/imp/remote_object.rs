use crate::imp::{connection::Connection, message};
use downcast_rs::{impl_downcast, DowncastSync};
use serde_json::value::Value;
use std::{pin::Pin, sync::Arc};
use strong::*;

pub(crate) trait RemoteObject: DowncastSync {
    fn channel_owner(&self) -> &ChannelOwner;
}
impl_downcast!(sync RemoteObject);

pub(crate) struct ChannelOwner {
    // TODO: Rc?
    parent: Option<Arc<ChannelOwner>>,
    typ: Str<message::ObjectType>,
    guid: Str<message::Guid>,
    initializer: Value
}

impl ChannelOwner {
    pub(crate) fn new(
        parent: Arc<ChannelOwner>,
        typ: Str<message::ObjectType>,
        guid: Str<message::Guid>,
        initializer: Value
    ) -> Self {
        Self {
            parent: Some(parent),
            typ,
            guid,
            initializer
        }
    }

    pub(crate) fn new_root() -> Self {
        Self {
            parent: None,
            typ: Str::validate("".into()).unwrap(),
            guid: Str::validate("".into()).unwrap(),
            initializer: Value::default()
        }
    }
}

pub(crate) struct DummyObject {
    channel: Arc<ChannelOwner>
}

impl DummyObject {
    fn new(channel: Arc<ChannelOwner>) -> Self { DummyObject { channel } }
}

impl RemoteObject for DummyObject {
    fn channel_owner(&self) -> &ChannelOwner { &self.channel }
}

pub(crate) fn create_remote_object(
    parent: Arc<ChannelOwner>,
    t: Str<message::ObjectType>,
    i: Str<message::Guid>,
    initializer: Value
) -> Arc<dyn RemoteObject> {
    let channel = Arc::new(ChannelOwner::new(parent, t, i, initializer));
    Arc::new(DummyObject::new(channel))
}
