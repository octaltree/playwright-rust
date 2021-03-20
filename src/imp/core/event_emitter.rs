pub(crate) use tokio::sync::broadcast;

pub trait EventEmitter {
    type Event: Clone;

    fn tx(&self) -> &broadcast::Sender<Self::Event>;

    fn subscribe_event(&self) -> broadcast::Receiver<Self::Event> { self.tx().subscribe() }

    fn emit_event<E: Into<Self::Event>>(&self, e: E) { self.tx().send(e.into()).ok(); }
}
