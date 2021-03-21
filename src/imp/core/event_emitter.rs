use crate::imp::{core::*, prelude::*};
pub(crate) use tokio::sync::{broadcast, broadcast::error::TryRecvError};

pub trait EventEmitter {
    type Event;

    fn tx(&self) -> &broadcast::Sender<Self::Event>;

    fn subscribe_event(&self) -> broadcast::Receiver<Self::Event> { self.tx().subscribe() }

    fn emit_event<E: Into<Self::Event>>(&self, e: E) { self.tx().send(e.into()).ok(); }
}

pub(crate) trait Event: Clone {
    type EventType: Clone + Copy + PartialEq;

    fn event_type(&self) -> Self::EventType;
}
