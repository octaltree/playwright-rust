use crate::imp::{core::*, prelude::*, request::Request, utils::Header};

#[derive(Debug)]
pub(crate) struct Route {
    channel: ChannelOwner,
    request: Weak<Request>
}

impl Route {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { request } = serde_json::from_value(channel.initializer.clone())?;
        let request = get_object!(ctx, &request.guid, Request)?;
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

    pub(crate) async fn fulfill(&self, args: FulfillArgs<'_, '_>) -> ArcResult<()> {
        let _ = send_message!(self, "fulfill", args);
        Ok(())
    }

    pub(crate) async fn r#continue(&self, args: ContinueArgs<'_, '_, '_>) -> ArcResult<()> {
        let _ = send_message!(self, "fulfill", args);
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FulfillArgs<'a, 'b> {
    body: &'a str,
    is_base64: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) headers: Option<Vec<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content_type: Option<&'b str>
}

impl<'a, 'b> FulfillArgs<'a, 'b> {
    pub(crate) fn new(body: &'a str, is_base64: bool) -> Self {
        Self {
            body,
            is_base64,
            status: None,
            headers: None,
            content_type: None
        }
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContinueArgs<'a, 'b, 'c> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) method: Option<&'b str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) headers: Option<Vec<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) post_data: Option<&'c str>
}
