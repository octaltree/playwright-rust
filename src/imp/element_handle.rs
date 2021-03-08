use crate::imp::{core::*, prelude::*};

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
        let s = first(&v)
            .ok_or(Error::InvalidParams)?
            .as_str()
            .ok_or(Error::InvalidParams)?;
        Ok(s.to_owned())
    }

    pub(crate) async fn inner_html(&self) -> ArcResult<String> {
        let v = send_message!(self, "innerHtml", Map::new());
        let s = first(&v)
            .ok_or(Error::InvalidParams)?
            .as_str()
            .ok_or(Error::InvalidParams)?;
        Ok(s.to_owned())
    }

    is_checked! {is_checked, "isChecked"}
    is_checked! {is_disabled, "isDisabled"}
    is_checked! {is_editable, "isEditable"}
    is_checked! {is_enabled, "isEnabled"}
    is_checked! {is_hidden, "isHidden"}
    is_checked! {is_visible, "isVisible"}
}

#[derive(Deserialize)]
struct QuerySelectorAllResponse {
    elements: Vec<OnlyGuid>
}

impl RemoteObject for ElementHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
