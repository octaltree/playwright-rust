use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct WebSocket {
    channel: ChannelOwner,
    url: String,
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug, Default)]
struct Variable {
    is_closed: bool
}

impl WebSocket {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { url } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            url,
            var: Mutex::default(),
            tx: Mutex::default()
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }
}

impl WebSocket {
    pub(crate) fn is_closed(&self) -> bool { self.var.lock().unwrap().is_closed }
}

impl RemoteObject for WebSocket {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        _ctx: &Context,
        method: Str<Method>,
        _params: Map<String, Value>
    ) -> Result<(), Error> {
        match method.as_str() {
            "frameSent" => {}
            "frameReceived" => {}
            "error" => {}
            "close" => {
                self.var.lock().unwrap().is_closed = true;
                self.emit_event(Evt::Close);
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    FrameSent,
    FrameReceived,
    Error,
    Close
}

impl EventEmitter for WebSocket {
    type Event = Evt;

    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().unwrap().clone() }

    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock().unwrap() = Some(tx); }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    FrameSent,
    FrameReceived,
    Error,
    Close
}

impl Event for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Evt::FrameSent => EventType::FrameSent,
            Evt::FrameReceived => EventType::FrameReceived,
            Evt::Error => EventType::Error,
            Evt::Close => EventType::Close
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String
}
