use crate::imp::{core::*, frame::Frame, prelude::*, response::Response, utils::Header};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner,
    url: String,
    resource_type: String,
    method: String,
    is_navigation_request: bool,
    frame: Weak<Frame>,
    post_data: Option<String>,
    headers: HashMap<String, String>,
    redirected_from: Option<Weak<Request>>
}

impl Request {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            url,
            resource_type,
            method,
            frame,
            is_navigation_request,
            post_data,
            headers,
            redirected_from
        } = serde_json::from_value(channel.initializer.clone())?;
        let headers: HashMap<_, _> = headers.into_iter().map(Into::<(_, _)>::into).collect();
        let frame = find_object!(ctx, &frame.guid, Frame)?;
        let redirected_from =
            match redirected_from.map(|OnlyGuid { guid }| find_object!(ctx, &guid, Request)) {
                None => None,
                Some(Ok(x)) => Some(x),
                Some(Err(e)) => return Err(e)
            };
        Ok(Self {
            channel,
            url,
            resource_type,
            method,
            frame,
            is_navigation_request,
            post_data,
            headers,
            redirected_from
        })
    }

    pub(crate) fn url(&self) -> &str { &self.url }

    pub(crate) fn resource_type(&self) -> &str { &self.resource_type }

    pub(crate) fn method(&self) -> &str { &self.method }

    pub(crate) fn is_navigation_request(&self) -> bool { self.is_navigation_request }

    pub(crate) fn frame(&self) -> Weak<Frame> { self.frame.clone() }

    pub(crate) fn post_data(&self) -> Option<Vec<u8>> {
        base64::decode(self.post_data.as_ref()?).ok()
    }

    pub(crate) fn post_data_as_string(&self) -> Option<String> {
        let bytes = self.post_data()?;
        let s = String::from_utf8(bytes).ok()?;
        Some(s)
    }

    pub(crate) fn headers(&self) -> &HashMap<String, String> { &self.headers }

    pub(crate) fn redirected_from(&self) -> Option<Weak<Request>> { self.redirected_from.clone() }

    pub(crate) async fn response(&self) -> ArcResult<Option<Weak<Response>>> {
        let v = send_message!(self, "response", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let r = find_object!(self.context()?.lock().unwrap(), &guid, Response)?;
        Ok(Some(r))
    }
}

// mutable
impl Request {
    // TODO: redirected_to
    // TODO: failure
    // TODO: timing
}

impl RemoteObject for Request {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    url: String,
    resource_type: String,
    method: String,
    frame: OnlyGuid,
    is_navigation_request: bool,
    // base64
    post_data: Option<String>,
    headers: Vec<Header>,
    redirected_from: Option<OnlyGuid>
}
