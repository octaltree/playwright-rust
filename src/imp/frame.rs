use crate::imp::{
    core::*,
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

impl Frame {
    pub(crate) fn new(channel: ChannelOwner) -> Self {
        let var = Mutex::new(Variable {});
        Self { channel, var }
    }

    pub(crate) async fn goto(
        &self,
        args: GotoArgs<'_, '_>
    ) -> Result<Option<Weak<Response>>, Arc<Error>> {
        let v = send_message!(self, "goto", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let r = find_object!(self.context()?.lock().unwrap(), &guid, Response)?;
        Ok(Some(r))
    }

    pub(crate) async fn click(&self, args: ClickArgs<'_>) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "click", args);
        Ok(())
    }

    pub(crate) async fn dblclick(&self, args: ClickArgs<'_>) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "dblclick", args);
        Ok(())
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

impl RemoteObject for Frame {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
