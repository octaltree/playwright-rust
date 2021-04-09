use crate::imp::{browser_context::BrowserContext, core::*, page::Page, prelude::*};

#[derive(Debug)]
pub(crate) struct Worker {
    channel: ChannelOwner,
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    page: Option<Weak<Page>>,
    browser_context: Option<Weak<BrowserContext>>
}

impl Worker {
    pub(crate) fn new(channel: ChannelOwner) -> Self {
        Self {
            channel,
            var: Mutex::default(),
            tx: Mutex::default()
        }
    }
}

impl Worker {
    pub(crate) fn set_page(&self, page: Weak<Page>) { self.var.lock().unwrap().page = Some(page); }

    // pub(crate) fn set_browser_context(&self, browser_context: Weak<BrowserContext>) {
    //    self.var.lock().unwrap().browser_context = Some(browser_context);
    //}

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let this = get_object!(ctx, self.guid(), Worker)?;
        let var = self.var.lock().unwrap();
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

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    Close
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Close
}

impl Event for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Self::Close => EventType::Close
        }
    }
}

impl EventEmitter for Worker {
    type Event = Evt;
    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().unwrap().clone() }
    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock().unwrap() = Some(tx); }
}
