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
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
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
    pub(crate) fn is_closed(&self) -> bool { self.var.lock().is_closed }

    fn on_frame_sent(&self, params: Map<String, Value>) -> Result<(), Error> {
        let buffer = parse_frame(params)?;
        self.emit_event(Evt::FrameSent(buffer));
        Ok(())
    }

    fn on_frame_received(&self, params: Map<String, Value>) -> Result<(), Error> {
        let buffer = parse_frame(params)?;
        self.emit_event(Evt::FrameReceived(buffer));
        Ok(())
    }
}

fn parse_frame(params: Map<String, Value>) -> Result<Buffer, Error> {
    #[derive(Deserialize)]
    struct De {
        opcode: i32,
        data: String
    }
    let De { opcode, data } = serde_json::from_value(params.into())?;
    let buffer = if opcode == 2 {
        let bytes = base64::decode(data).map_err(Error::InvalidBase64)?;
        Buffer::Bytes(bytes)
    } else {
        Buffer::String(data)
    };
    Ok(buffer)
}

impl RemoteObject for WebSocket {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        _ctx: &Context,
        method: Str<Method>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        match method.as_str() {
            "framesent" => self.on_frame_sent(params)?,
            "framereceived" => self.on_frame_received(params)?,
            "error" => {
                let error: Value = params.get("error").cloned().unwrap_or_default();
                self.emit_event(Evt::Error(error));
            }
            "close" => {
                self.var.lock().is_closed = true;
                self.emit_event(Evt::Close);
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    FrameSent(Buffer),
    FrameReceived(Buffer),
    Error(Value),
    Close
}

#[derive(Debug, Clone)]
pub enum Buffer {
    Bytes(Vec<u8>),
    String(String)
}

impl EventEmitter for WebSocket {
    type Event = Evt;

    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().clone() }

    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock() = Some(tx); }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    FrameSent,
    FrameReceived,
    Error,
    Close
}

impl IsEvent for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Evt::FrameSent(_) => EventType::FrameSent,
            Evt::FrameReceived(_) => EventType::FrameReceived,
            Evt::Error(_) => EventType::Error,
            Evt::Close => EventType::Close
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String
}
