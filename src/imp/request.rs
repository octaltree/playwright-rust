use crate::imp::{
    core::*,
    frame::Frame,
    prelude::*,
    response::Response,
    utils::{Header, ResponseTiming}
};

#[derive(Debug)]
pub(crate) struct Request {
    channel: ChannelOwner,
    url: String,
    resource_type: String,
    method: String,
    is_navigation_request: bool,
    post_data: Option<String>,
    frame: Weak<Frame>,
    headers: HashMap<String, String>,
    redirected_from: Option<Weak<Request>>,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    redirected_to: Option<Weak<Request>>,
    failure: Option<String>,
    timing: Option<ResponseTiming>,
    response_end: Option<f64>
}

impl Request {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Arc<Self>, Error> {
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
        let headers: HashMap<_, _> = headers
            .into_iter()
            .map(Into::<(_, _)>::into)
            .map(|(mut k, v)| {
                k.make_ascii_lowercase();
                (k, v)
            })
            .collect();
        let frame = get_object!(ctx, &frame.guid, Frame)?;
        let redirected_from =
            match redirected_from.map(|OnlyGuid { guid }| get_object!(ctx, &guid, Request)) {
                None => None,
                Some(Ok(x)) => Some(x),
                Some(Err(e)) => return Err(e)
            };
        let var = Mutex::new(Variable::default());
        let arc = Arc::new(Self {
            channel,
            url,
            resource_type,
            method,
            is_navigation_request,
            post_data,
            frame,
            headers,
            redirected_from,
            var
        });
        if let Some(from) = arc.redirected_from.as_ref().and_then(|w| w.upgrade()) {
            let this = Arc::downgrade(&arc);
            from.set_redirected_to(this);
        }
        Ok(arc)
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
        let r = get_object!(self.context()?.lock(), guid, Response)?;
        Ok(Some(r))
    }
}

impl Request {
    pub(crate) fn timing(&self) -> Option<ResponseTiming> {
        self.var.lock().timing.clone()
    }

    pub(crate) fn response_end(&self) -> Option<f64> { self.var.lock().response_end }

    pub(crate) fn failure(&self) -> Option<String> { self.var.lock().failure.clone() }

    pub(crate) fn redirected_to(&self) -> Option<Weak<Request>> {
        self.var.lock().redirected_to.clone()
    }

    fn set_redirected_to(&self, to: Weak<Request>) {
        let var = &mut self.var.lock();
        var.redirected_to = Some(to);
    }

    pub(crate) fn set_response_timing(&self, timing: ResponseTiming) {
        let var = &mut self.var.lock();
        var.timing = Some(timing);
    }

    pub(crate) fn set_response_end(&self, response_end: f64) {
        let var = &mut self.var.lock();
        var.response_end = Some(response_end);
    }

    pub(crate) fn set_failure(&self, failure: Option<String>) {
        let var = &mut self.var.lock();
        var.failure = failure;
    }
}

// mutable
impl Request {
    // redirected_to
    // failure
    // timing
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
