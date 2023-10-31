use base64::Engine;
use crate::imp::{
    core::*,
    prelude::*,
    request::Request,
    utils::{Header, ResponseTiming}
};

#[derive(Debug)]
pub(crate) struct Response {
    channel: ChannelOwner,
    url: String,
    status: i32,
    status_text: String,
    request: Weak<Request>
}

impl Response {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            status,
            status_text,
            request,
            timing
        } = serde_json::from_value(channel.initializer.clone())?;
        let request = get_object!(ctx, &request.guid, Request)?;
        upgrade(&request)?.set_response_timing(timing);
        Ok(Self {
            channel,
            url,
            status,
            status_text,
            request
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }
    pub(crate) fn status(&self) -> i32 { self.status }
    pub(crate) fn status_text(&self) -> &str { &self.status_text }

    pub(crate) fn ok(&self) -> bool { self.status == 0 || (200..300).contains(&self.status) }

    pub(crate) async fn finished(&self) -> ArcResult<Option<String>> {
        let v = send_message!(self, "finished", Map::new());
        let s = maybe_only_str(&v)?;
        Ok(s.map(ToOwned::to_owned))
    }

    pub(crate) async fn body(&self) -> ArcResult<Vec<u8>> {
        let v = send_message!(self, "body", Map::new());
        let s = only_str(&v)?;
        let bytes = base64::engine::general_purpose::STANDARD.decode(s).map_err(Error::InvalidBase64)?;
        Ok(bytes)
    }

    pub(crate) async fn text(&self) -> ArcResult<String> {
        Ok(String::from_utf8(self.body().await?).map_err(Error::InvalidUtf8)?)
    }

    pub(crate) fn request(&self) -> Weak<Request> { self.request.clone() }

    pub(crate) async fn headers(&self) -> ArcResult<Vec<Header>> {
        let v = send_message!(self, "body", Map::new());
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let mut headers: Vec<Header> =
            serde_json::from_value((*first).clone()).map_err(Error::Serde)?;
        for h in headers.iter_mut() {
            h.name.make_ascii_lowercase();
        }
        Ok(headers)
    }
}

impl RemoteObject for Response {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String,
    status: i32,
    status_text: String,
    request: OnlyGuid,
    timing: ResponseTiming
}
