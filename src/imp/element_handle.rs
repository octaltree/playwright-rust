use crate::imp::{
    core::*,
    frame::Frame,
    prelude::*,
    utils::{KeyboardModifier, MouseButton, Position}
};

#[derive(Debug)]
pub(crate) struct ElementHandle {
    channel: ChannelOwner
}

macro_rules! is_checked {
    ($f: ident, $m: literal) => {
        pub(crate) async fn $f(&self) -> ArcResult<bool> {
            let v = send_message!(self, $m, Map::new());
            let b = first(&v)
                .ok_or(Error::InvalidParams)?
                .as_bool()
                .ok_or(Error::InvalidParams)?;
            Ok(b)
        }
    };
}

impl ElementHandle {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

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

    pub(crate) async fn inner_text(&self) -> ArcResult<String> {
        let v = send_message!(self, "innerText", Map::new());
        let s = only_str(&v)?;
        Ok(s.to_owned())
    }

    pub(crate) async fn inner_html(&self) -> ArcResult<String> {
        let v = send_message!(self, "innerHtml", Map::new());
        let s = only_str(&v)?;
        Ok(s.to_owned())
    }

    is_checked! {is_checked, "isChecked"}
    is_checked! {is_disabled, "isDisabled"}
    is_checked! {is_editable, "isEditable"}
    is_checked! {is_enabled, "isEnabled"}
    is_checked! {is_hidden, "isHidden"}
    is_checked! {is_visible, "isVisible"}

    pub(crate) async fn owner_frame(&self) -> ArcResult<Option<Weak<Frame>>> {
        let v = send_message!(self, "ownerFrame", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let f = find_object!(self.context()?.lock().unwrap(), &guid, Frame)?;
        Ok(Some(f))
    }

    pub(crate) async fn content_frame(&self) -> ArcResult<Option<Weak<Frame>>> {
        let v = send_message!(self, "contentFrame", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let f = find_object!(self.context()?.lock().unwrap(), &guid, Frame)?;
        Ok(Some(f))
    }

    pub(crate) async fn get_attribute(&self, name: &str) -> ArcResult<Option<String>> {
        let mut args = HashMap::new();
        args.insert("name", name);
        let v = send_message!(self, "getAttribute", args);
        let s = maybe_only_str(&v)?;
        Ok(s.map(ToOwned::to_owned))
    }

    pub(crate) async fn text_content(&self) -> ArcResult<Option<String>> {
        let v = send_message!(self, "textContent", Map::new());
        let s = maybe_only_str(&v)?;
        Ok(s.map(ToOwned::to_owned))
    }

    pub(crate) async fn hover(&self, args: HoverArgs) -> ArcResult<()> {
        let _ = send_message!(self, "hover", args);
        Ok(())
    }

    pub(crate) async fn click(&self, args: ClickArgs) -> ArcResult<()> {
        let _ = send_message!(self, "click", args);
        Ok(())
    }

    pub(crate) async fn dblclick(&self, args: ClickArgs) -> ArcResult<()> {
        let _ = send_message!(self, "dblclick", args);
        Ok(())
    }

    pub(crate) async fn check(&self, args: CheckArgs) -> ArcResult<()> {
        let _ = send_message!(self, "check", args);
        Ok(())
    }

    pub(crate) async fn uncheck(&self, args: CheckArgs) -> ArcResult<()> {
        let _ = send_message!(self, "uncheck", args);
        Ok(())
    }
}

#[derive(Deserialize)]
struct QuerySelectorAllResponse {
    elements: Vec<OnlyGuid>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HoverArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClickArgs {
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

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) no_wait_after: Option<bool>
}

impl RemoteObject for ElementHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
