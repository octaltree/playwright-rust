use crate::imp::{
    browser_context::BrowserContext, core::*, js_handle::JsHandle, page::Page, prelude::*
};

#[derive(Debug)]
pub(crate) struct Worker {
    channel: ChannelOwner,
    url: String,
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    page: Option<Weak<Page>>,
    browser_context: Option<Weak<BrowserContext>>
}

impl Worker {
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

    pub(crate) async fn eval<U>(&self, expression: &str) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        self.evaluate::<(), U>(expression, None).await
    }

    pub(crate) async fn evaluate<T, U>(&self, expression: &str, arg: Option<T>) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            expression: &'a str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args { expression, arg };
        let v = send_message!(self, "evaluateExpression", args);
        let first = first(&v).ok_or(Error::ObjectNotFound)?;
        Ok(de::from_value(first).map_err(Error::DeserializationPwJson)?)
    }

    pub(crate) async fn eval_handle(&self, expression: &str) -> ArcResult<Weak<JsHandle>> {
        self.evaluate_handle::<()>(expression, None).await
    }

    pub(crate) async fn evaluate_handle<T>(
        &self,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<Weak<JsHandle>>
    where
        T: Serialize
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            expression: &'a str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args { expression, arg };
        let v = send_message!(self, "evaluateExpressionHandle", args);
        let guid = only_guid(&v)?;
        let h = get_object!(self.context()?.lock(), guid, JsHandle)?;
        Ok(h)
    }
}

impl Worker {
    pub(crate) fn set_page(&self, page: Weak<Page>) { self.var.lock().page = Some(page); }

    // pub(crate) fn set_browser_context(&self, browser_context: Weak<BrowserContext>) {
    //    self.var.lock().browser_context = Some(browser_context);
    //}

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let this = get_object!(ctx, self.guid(), Worker)?;
        let var = self.var.lock();
        if let Some(page) = var.page.as_ref().and_then(Weak::upgrade) {
            page.remove_worker(&this);
        }
        // var.context.remove_service_worker(&this)
        self.emit_event(Evt::Close);
        Ok(())
    }
}

impl RemoteObject for Worker {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        ctx: &Context,
        method: Str<Method>,
        _params: Map<String, Value>
    ) -> Result<(), Error> {
        if method.as_str() == "close" {
            self.on_close(ctx)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    Close
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Close
}

impl IsEvent for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Self::Close => EventType::Close
        }
    }
}

impl EventEmitter for Worker {
    type Event = Evt;
    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().clone() }
    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock() = Some(tx); }
}
