use crate::imp::{core::*, prelude::*, request::Request};

#[derive(Debug)]
pub(crate) struct Route {
    channel: ChannelOwner,
    request: Weak<Request>
}

impl Route {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { request } = serde_json::from_value(channel.initializer.clone())?;
        let request = find_object!(ctx, &request.guid, Request)?;
        Ok(Self { channel, request })
    }

    pub(crate) fn request(&self) -> Weak<Request> { self.request.clone() }

    pub(crate) async fn abort(&self, err_code: Option<&str>) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        if let Some(x) = err_code {
            args.insert("errCode", x);
        }
        let _ = send_message!(self, "abort", args);
        Ok(())
    }
}

impl RemoteObject for Route {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    request: OnlyGuid
}
