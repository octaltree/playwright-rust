use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct JsHandle {
    channel: ChannelOwner,
    var: Mutex<Var>
}

#[derive(Debug)]
struct Var {
    preview: String
}

impl JsHandle {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { preview } = serde_json::from_value(channel.initializer.clone())?;
        let var = Mutex::new(Var { preview });
        Ok(Self { channel, var })
    }
}

impl RemoteObject for JsHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    preview: String
}
