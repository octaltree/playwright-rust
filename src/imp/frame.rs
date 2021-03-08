use crate::imp::{
    core::*,
    element_handle::ElementHandle,
    prelude::*,
    response::Response,
    utils::{DocumentLoadState, KeyboardModifier, MouseButton, Position}
};

#[derive(Debug)]
pub(crate) struct Frame {
    channel: ChannelOwner,
    var: Mutex<Variable>
}

#[derive(Debug)]
struct Variable {}

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
    pub(crate) fn new(channel: ChannelOwner) -> Self {
        let var = Mutex::new(Variable {});
        Self { channel, var }
    }

    pub(crate) async fn goto(&self, args: GotoArgs<'_, '_>) -> ArcResult<Option<Weak<Response>>> {
        let v = send_message!(self, "goto", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let r = find_object!(self.context()?.lock().unwrap(), &guid, Response)?;
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
        let e = find_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn query_selector_all(
        &self,
        selector: &str
    ) -> ArcResult<Vec<Weak<ElementHandle>>> {
        let mut args = HashMap::new();
        args.insert("selector", selector);
        let v = send_message!(self, "querySelectorAll", args);
        let QuerySelectorAllResponse { elements } =
            serde_json::from_value((*v).clone()).map_err(Error::Serde)?;
        let es = elements
            .into_iter()
            .map(|OnlyGuid { guid }| {
                find_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(es)
    }

    pub(crate) async fn frame_element(&self) -> ArcResult<Weak<ElementHandle>> {
        let v = send_message!(self, "frameElement", Map::new());
        let guid = only_guid(&v)?;
        let e = find_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
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
        let e = find_object!(self.context()?.lock().unwrap(), &guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn title(&self) -> ArcResult<String> {
        let v = send_message!(self, "title", Map::new());
        let s = first(&v)
            .ok_or(Error::InvalidParams)?
            .as_str()
            .ok_or(Error::InvalidParams)?;
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

    pub(crate) async fn set_content(&self, args: SetContentArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "setContent", args);
        Ok(())
    }
}

#[derive(Deserialize)]
struct QuerySelectorAllResponse {
    elements: Vec<OnlyGuid>
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

impl RemoteObject for Frame {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
