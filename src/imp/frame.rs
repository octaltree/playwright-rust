use crate::imp::{core::*, prelude::*, response::Response, utils::DocumentLoadState};

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

impl RemoteObject for Frame {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
