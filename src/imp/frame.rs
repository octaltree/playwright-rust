pub(crate) use crate::imp::element_handle::Opt;
use crate::imp::{
    core::*,
    element_handle::ElementHandle,
    js_handle::JsHandle,
    page::Page,
    prelude::*,
    response::Response,
    utils::{DocumentLoadState, File, KeyboardModifier, MouseButton, Position}
};
use std::{collections::HashSet, iter::FromIterator};

#[derive(Debug)]
pub(crate) struct Frame {
    channel: ChannelOwner,
    parent_frame: Option<Weak<Frame>>,
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug)]
struct Variable {
    url: String,
    name: String,
    page: Option<Weak<Page>>,
    child_frames: Vec<Weak<Frame>>,
    load_states: HashSet<DocumentLoadState>
}

macro_rules! is_checked {
    ($f: ident, $m: literal) => {
        pub(crate) async fn $f(&self, selector: &str, timeout: Option<f64>) -> ArcResult<bool> {
            #[derive(Serialize)]
            struct Args<'a> {
                selector: &'a str,
                #[serde(skip_serializing_if = "Option::is_none")]
                timeout: Option<f64>
            }
            let args = Args { selector, timeout };
            let v = send_message!(self, $m, args);
            let b = first(&v)
                .ok_or(Error::InvalidParams)?
                .as_bool()
                .ok_or(Error::InvalidParams)?;
            Ok(b)
        }
    };
}

impl Frame {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            name,
            url,
            parent_frame,
            load_states
        } = serde_json::from_value(channel.initializer.clone())?;
        let parent_frame =
            match parent_frame.map(|OnlyGuid { guid }| get_object!(ctx, &guid, Frame)) {
                Some(Err(e)) => return Err(e),
                Some(Ok(x)) => Some(x),
                None => None
            };
        let var = Mutex::new(Variable {
            url,
            name,
            page: None,
            child_frames: Vec::new(),
            load_states: HashSet::from_iter(load_states)
        });
        Ok(Self {
            channel,
            parent_frame,
            var,
            tx: Mutex::default()
        })
    }

    pub(crate) fn hook_created(&self, this: Weak<Frame>) -> Result<(), Error> {
        if let Some(parent) = &self.parent_frame {
            upgrade(parent)?.add_child_frames(this);
        }
        Ok(())
    }

    pub(crate) async fn goto(&self, args: GotoArgs<'_, '_>) -> ArcResult<Option<Weak<Response>>> {
        let v = send_message!(self, "goto", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let r = get_object!(self.context()?.lock().unwrap(), &guid, Response)?;
        Ok(Some(r))
    }

    pub(crate) async fn click(&self, args: ClickArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "click", args);
        Ok(())
    }

    pub(crate) async fn dblclick(&self, args: ClickArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "dblclick", args);
        Ok(())
    }

    pub(crate) async fn tap(&self, args: TapArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "tap", args);
        Ok(())
    }

    pub(crate) async fn fill(&self, args: FillArgs<'_, '_>) -> ArcResult<()> {
        let _ = send_message!(self, "fill", args);
        Ok(())
    }

    pub(crate) async fn focus(&self, selector: &str, timeout: Option<f64>) -> ArcResult<()> {
        let args = SelectorTimeout { selector, timeout };
        let _ = send_message!(self, "focus", args);
        Ok(())
    }

    pub(crate) async fn text_content(
        &self,
        selector: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        let args = SelectorTimeout { selector, timeout };
        let v = send_message!(self, "textContent", args);
        let s = maybe_only_str(&v)?;
        Ok(s.map(Into::into))
    }

    pub(crate) async fn inner_text(
        &self,
        selector: &str,
        timeout: Option<f64>
    ) -> ArcResult<String> {
        let args = SelectorTimeout { selector, timeout };
        let v = send_message!(self, "innerText", args);
        let s = only_str(&v)?;
        Ok(s.into())
    }

    pub(crate) async fn inner_html(
        &self,
        selector: &str,
        timeout: Option<f64>
    ) -> ArcResult<String> {
        let args = SelectorTimeout { selector, timeout };
        let v = send_message!(self, "innerHtml", args);
        let s = only_str(&v)?;
        Ok(s.into())
    }

    pub(crate) async fn get_attribute(
        &self,
        selector: &str,
        name: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        #[derive(Serialize)]
        struct Args<'a, 'b> {
            selector: &'a str,
            name: &'b str,
            #[serde(skip_serializing_if = "Option::is_none")]
            timeout: Option<f64>
        }
        let args = Args {
            selector,
            name,
            timeout
        };
        let v = send_message!(self, "getAttribute", args);
        let s = maybe_only_str(&v)?;
        Ok(s.map(Into::into))
    }

    pub(crate) async fn query_selector(
        &self,
        selector: &str
    ) -> ArcResult<Option<Weak<ElementHandle>>> {
        let mut args = HashMap::new();
        args.insert("selector", selector);
        let v = send_message!(self, "querySelector", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let e = get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn query_selector_all(
        &self,
        selector: &str
    ) -> ArcResult<Vec<Weak<ElementHandle>>> {
        let mut args = HashMap::new();
        args.insert("selector", selector);
        let v = send_message!(self, "querySelectorAll", args);
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let elements: Vec<OnlyGuid> =
            serde_json::from_value((*first).clone()).map_err(Error::Serde)?;
        let es = elements
            .into_iter()
            .map(|OnlyGuid { guid }| {
                get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(es)
    }

    pub(crate) async fn frame_element(&self) -> ArcResult<Weak<ElementHandle>> {
        let v = send_message!(self, "frameElement", Map::new());
        let guid = only_guid(&v)?;
        let e = get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(e)
    }

    pub(crate) async fn wait_for_selector(
        &self,
        args: WaitForSelectorArgs<'_>
    ) -> ArcResult<Option<Weak<ElementHandle>>> {
        let v = send_message!(self, "WaitForSelector", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let e = get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn title(&self) -> ArcResult<String> {
        let v = send_message!(self, "title", Map::new());
        let s = only_str(&v)?;
        Ok(s.to_owned())
    }

    pub(crate) async fn r#type(&self, args: TypeArgs<'_, '_>) -> ArcResult<()> {
        let _ = send_message!(self, "type", args);
        Ok(())
    }

    pub(crate) async fn press(&self, args: PressArgs<'_, '_>) -> ArcResult<()> {
        let _ = send_message!(self, "press", args);
        Ok(())
    }

    pub(crate) async fn hover(&self, args: HoverArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "hover", args);
        Ok(())
    }

    is_checked!(is_checked, "isChecked");
    is_checked!(is_disabled, "isDisabled");
    is_checked!(is_editable, "isEditable");
    is_checked!(is_enabled, "isEnabled");
    is_checked!(is_hidden, "isHidden");
    is_checked!(is_visible, "isVisible");

    pub(crate) async fn content(&self) -> ArcResult<String> {
        let v = send_message!(self, "content", Map::new());
        let s = only_str(&v)?;
        Ok(s.into())
    }

    pub(crate) async fn set_content(&self, args: SetContentArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "setContent", args);
        Ok(())
    }

    pub(crate) async fn check(&self, args: CheckArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "check", args);
        Ok(())
    }

    pub(crate) async fn uncheck(&self, args: CheckArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "uncheck", args);
        Ok(())
    }

    pub(crate) async fn add_script_tag(
        &self,
        args: AddScriptTagArgs<'_, '_, '_>
    ) -> ArcResult<Weak<ElementHandle>> {
        let v = send_message!(self, "addScriptTag", args);
        let guid = only_guid(&v)?;
        let e = get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(e)
    }

    pub(crate) async fn add_style_tag(
        &self,
        content: &str,
        url: Option<&str>
    ) -> ArcResult<Weak<ElementHandle>> {
        let mut args = HashMap::new();
        args.insert("content", content);
        if let Some(url) = url {
            args.insert("url", url);
        }
        let v = send_message!(self, "addStyleTag", args);
        let guid = only_guid(&v)?;
        let e = get_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(e)
    }

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
        struct Args<'a> {
            expression: &'a str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args { expression, arg };
        let v = send_message!(self, "evaluateExpression", args);
        let first = first(&v).ok_or(Error::ObjectNotFound)?;
        Ok(de::from_value(&first).map_err(Error::DeserializationPwJson)?)
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
        struct Args<'a> {
            expression: &'a str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args { expression, arg };
        let v = send_message!(self, "evaluateExpressionHandle", args);
        let guid = only_guid(&v)?;
        let h = get_object!(self.context()?.lock().unwrap(), &guid, JsHandle)?;
        Ok(h)
    }

    pub(crate) async fn eval_on_selector<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        #[derive(Serialize)]
        struct Args<'a, 'b> {
            selector: &'a str,
            expression: &'b str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args {
            selector,
            expression,
            arg
        };
        let v = send_message!(self, "evalOnSelector", args);
        let first = first(&v).ok_or(Error::ObjectNotFound)?;
        Ok(de::from_value(&first).map_err(Error::DeserializationPwJson)?)
    }

    pub(crate) async fn eval_on_selector_all<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        #[derive(Serialize)]
        struct Args<'a, 'b> {
            selector: &'a str,
            expression: &'b str,
            arg: Value
        }
        let arg = ser::to_value(&arg).map_err(Error::SerializationPwJson)?;
        let args = Args {
            selector,
            expression,
            arg
        };
        let v = send_message!(self, "evalOnSelectorAll", args);
        let first = first(&v).ok_or(Error::ObjectNotFound)?;
        Ok(de::from_value(&first).map_err(Error::DeserializationPwJson)?)
    }

    pub(crate) async fn dispatch_event<T>(
        &self,
        selector: &str,
        r#type: &str,
        event_init: Option<T>
    ) -> ArcResult<()>
    where
        T: Serialize
    {
        #[derive(Serialize)]
        struct Args<'a, 'b> {
            selector: &'a str,
            r#type: &'b str,
            event_init: Value
        }
        let event_init = ser::to_value(&event_init).map_err(Error::SerializationPwJson)?;
        let args = Args {
            selector,
            r#type,
            event_init
        };
        let _ = send_message!(self, "dispatchEvent", args);
        Ok(())
    }

    pub(crate) async fn select_option(&self, args: SelectOptionArgs<'_>) -> ArcResult<Vec<String>> {
        let v = send_message!(self, "selectOption", args);
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let ss = first
            .as_array()
            .ok_or(Error::InvalidParams)?
            .iter()
            .filter_map(|v| v.as_str())
            .map(ToOwned::to_owned)
            .collect();
        Ok(ss)
    }

    pub(crate) async fn set_input_files(&self, args: SetInputFilesArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "setInputFiles", args);
        Ok(())
    }

    pub(crate) async fn wait_for_function(
        &self,
        args: WaitForFunctionArgs<'_>
    ) -> ArcResult<Weak<JsHandle>> {
        let v = send_message!(self, "waitForFunction", args);
        let guid = only_guid(&v)?;
        let h = get_object!(self.context()?.lock().unwrap(), &guid, JsHandle)?;
        Ok(h)
    }
}

// mutable
impl Frame {
    pub(crate) fn url(&self) -> String { self.var.lock().unwrap().url.clone() }

    pub(crate) fn name(&self) -> String { self.var.lock().unwrap().name.clone() }

    pub(crate) fn page(&self) -> Option<Weak<Page>> { self.var.lock().unwrap().page.clone() }

    pub(crate) fn set_page(&self, page: Weak<Page>) { self.var.lock().unwrap().page = Some(page); }

    pub(crate) fn parent_frame(&self) -> Option<Weak<Frame>> { self.parent_frame.clone() }

    pub(crate) fn child_frames(&self) -> Vec<Weak<Frame>> {
        self.var.lock().unwrap().child_frames.clone()
    }

    pub(crate) fn add_child_frames(&self, child: Weak<Frame>) {
        self.var.lock().unwrap().child_frames.push(child);
    }

    fn on_navigated(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct New {
            name: String,
            url: String
        }
        let New { name, url } = serde_json::from_value(params.clone().into())?;
        let var = &mut self.var.lock().unwrap();
        var.name = name;
        var.url = url;
        let payload: FrameNavigatedEvent = serde_json::from_value(params.into())?;
        self.emit_event(Evt::Navigated(payload));
        if let Some(page) = var.page.as_ref().and_then(|p| p.upgrade()) {
            let this = get_object!(ctx, &self.guid(), Frame)?;
            page.on_frame_navigated(this);
        }
        Ok(())
    }

    fn on_load_state(&self, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum Op {
            Add(DocumentLoadState),
            Remove(DocumentLoadState)
        }
        let op: Op = serde_json::from_value(params.into())?;
        let load_states = &mut self.var.lock().unwrap().load_states;
        match op {
            Op::Add(x) => {
                load_states.insert(x);
                self.emit_event(Evt::LoadState(x));
            }
            Op::Remove(x) => {
                load_states.remove(&x);
            }
        }
        Ok(())
    }
}

impl RemoteObject for Frame {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        ctx: &Context,
        method: Str<Method>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        match method.as_str() {
            "navigated" => self.on_navigated(ctx, params)?,
            "loadstate" => self.on_load_state(params)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    LoadState(DocumentLoadState),
    Navigated(FrameNavigatedEvent)
}

impl EventEmitter for Frame {
    type Event = Evt;

    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().unwrap().clone() }

    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock().unwrap() = Some(tx); }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    LoadState,
    Navigated
}

impl Event for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Evt::LoadState(_) => EventType::LoadState,
            Evt::Navigated(_) => EventType::Navigated
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GotoArgs<'a, 'b> {
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wait_until: Option<DocumentLoadState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) referer: Option<&'b str>
}

impl<'a> GotoArgs<'a, '_> {
    pub(crate) fn new(url: &'a str) -> Self {
        Self {
            url,
            timeout: None,
            wait_until: None,
            referer: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClickArgs<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) button: Option<MouseButton>,
    /// Is ignored if dblclick
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) click_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a> ClickArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            modifiers: None,
            position: None,
            delay: None,
            button: None,
            /// Is ignored if dblclick
            click_count: None,
            timeout: None,
            force: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WaitForSelectorArgs<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) state: Option<FrameState>
}

impl<'a> WaitForSelectorArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            timeout: None,
            state: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FrameState {
    Attached,
    Detached,
    Hidden,
    Visible
}

macro_rules! type_args {
    ($t:ident, $f:ident) => {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub(crate) struct $t<'a, 'b> {
            selector: &'a str,
            $f: &'b str,
            pub(crate) delay: Option<f64>,
            pub(crate) timeout: Option<f64>,
            pub(crate) no_wait_after: Option<bool>
        }

        impl<'a, 'b> $t<'a, 'b> {
            pub(crate) fn new(selector: &'a str, $f: &'b str) -> Self {
                Self {
                    selector,
                    $f,
                    delay: None,
                    timeout: None,
                    no_wait_after: None
                }
            }
        }
    };
}

type_args! {TypeArgs, text}
type_args! {PressArgs, key}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HoverArgs<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>
}

impl<'a> HoverArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            modifiers: None,
            position: None,
            timeout: None,
            force: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetContentArgs<'a> {
    html: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wait_until: Option<DocumentLoadState>
}

impl<'a> SetContentArgs<'a> {
    pub(crate) fn new(html: &'a str) -> Self {
        Self {
            html,
            timeout: None,
            wait_until: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TapArgs<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a> TapArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            modifiers: None,
            position: None,
            timeout: None,
            force: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FillArgs<'a, 'b> {
    selector: &'a str,
    value: &'b str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a, 'b> FillArgs<'a, 'b> {
    pub(crate) fn new(selector: &'a str, value: &'b str) -> Self {
        Self {
            selector,
            value,
            timeout: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SelectorTimeout<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<f64>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckArgs<'a> {
    selector: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a> CheckArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            timeout: None,
            force: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddScriptTagArgs<'a, 'b, 'c> {
    content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<&'b str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) r#type: Option<&'c str>
}

impl<'a, 'b, 'c> AddScriptTagArgs<'a, 'b, 'c> {
    pub(crate) fn new(content: &'a str) -> Self {
        Self {
            content,
            url: None,
            r#type: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SelectOptionArgs<'a> {
    selector: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) options: Option<Vec<Opt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) elements: Option<Vec<OnlyGuid>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a> SelectOptionArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            options: None,
            elements: None,
            timeout: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetInputFilesArgs<'a> {
    selector: &'a str,

    pub(crate) files: Vec<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl<'a> SetInputFilesArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            files: Vec::new(),
            timeout: None,
            no_wait_after: None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WaitForFunctionArgs<'a> {
    expression: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) polling: Option<Polling>,
    pub(crate) arg: Option<Value>
}

pub enum Polling {
    RequestAnimationFrame,
    Millis(u32)
}

impl Serialize for Polling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer
    {
        match self {
            Self::Millis(x) => x.serialize(serializer),
            Self::RequestAnimationFrame => "raf".serialize(serializer)
        }
    }
}

impl<'a> WaitForFunctionArgs<'a> {
    pub(crate) fn new(expression: &'a str) -> Self {
        Self {
            expression,
            timeout: None,
            polling: None,
            arg: None
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    name: String,
    url: String,
    parent_frame: Option<OnlyGuid>,
    load_states: Vec<DocumentLoadState>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameNavigatedEvent {
    url: String,
    name: String,
    new_document: Option<Document>,
    error: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    request: Value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::{
        browser::{Browser, *},
        browser_context::BrowserContext,
        browser_type::*,
        page::Page,
        playwright::Playwright
    };

    crate::runtime_test!(eval_handle, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let pw = Playwright::wait_initial_object(&conn).await.unwrap();
        let pw: Arc<Playwright> = pw.upgrade().unwrap();
        let chromium: Arc<BrowserType> = pw.chromium().upgrade().unwrap();
        let browser: Weak<Browser> = chromium.launch(LaunchArgs::default()).await.unwrap();
        let browser: Arc<Browser> = browser.upgrade().unwrap();
        let browser_context: Weak<BrowserContext> = browser
            .new_context(NewContextArgs::default())
            .await
            .unwrap();
        let browser_context: Arc<BrowserContext> = browser_context.upgrade().unwrap();
        let page: Weak<Page> = browser_context.new_page().await.unwrap();
        let page: Arc<Page> = page.upgrade().unwrap();
        let frame: Weak<Frame> = page.main_frame();
        let frame: Arc<Frame> = frame.upgrade().unwrap();
        let _handle: Weak<JsHandle> = frame.eval_handle("() => location.href").await.unwrap();
    });

    #[test]
    fn serialize_enum() {
        let s = serde_json::to_string(&Polling::Millis(3)).unwrap();
        assert_eq!(s, "3");
        let s = serde_json::to_string(&Polling::RequestAnimationFrame).unwrap();
        assert_eq!(s, r#""raf""#);
    }
}
