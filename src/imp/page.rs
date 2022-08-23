use crate::imp::{
    browser_context::BrowserContext,
    console_message::ConsoleMessage,
    core::*,
    download::Download,
    file_hooser::FileChooser,
    frame::Frame,
    prelude::*,
    request::Request,
    response::Response,
    utils::{
        ColorScheme, DocumentLoadState, FloatRect, Header, Length, MouseButton, PdfMargins,
        ScreenshotType, Viewport
    },
    video::Video,
    websocket::WebSocket,
    worker::Worker
};
use crate::protocol::generated::LifecycleEvent;

#[derive(Debug)]
pub(crate) struct Page {
    channel: ChannelOwner,
    main_frame: Weak<Frame>,
    browser_context: Weak<BrowserContext>,
    var: Mutex<Variable>,
    tx: Mutex<Option<broadcast::Sender<Evt>>>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    viewport: Option<Viewport>,
    frames: Vec<Weak<Frame>>,
    timeout: Option<u32>,
    navigation_timeout: Option<u32>,
    workers: Vec<Weak<Worker>>,
    video: Option<Video>
}

macro_rules! navigation {
    ($f: ident, $m: literal) => {
        pub(crate) async fn $f(
            &self,
            args: ReloadArgs
        ) -> Result<Option<Weak<Response>>, Arc<Error>> {
            let v = send_message!(self, $m, args);
            let guid = match as_only_guid(&v) {
                Some(g) => g,
                None => return Ok(None)
            };
            let r = get_object!(self.context()?.lock().unwrap(), &guid, Response)?;
            Ok(Some(r))
        }
    };
}

macro_rules! mouse_down {
    ($f:ident, $m:literal) => {
        pub(crate) async fn $f(
            &self,
            button: Option<MouseButton>,
            click_count: Option<i32>
        ) -> Result<(), Arc<Error>> {
            #[skip_serializing_none]
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
                button: Option<MouseButton>,
                click_count: Option<i32>
            }
            let args = Args {
                button,
                click_count
            };
            let _ = send_message!(self, $m, args);
            Ok(())
        }
    };
}

impl Page {
    const DEFAULT_TIMEOUT: u32 = 30000;

    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            main_frame: OnlyGuid { guid },
            viewport
        } = serde_json::from_value(channel.initializer.clone())?;
        let browser_context = match &channel.parent {
            Some(RemoteWeak::BrowserContext(c)) => c.clone(),
            _ => return Err(Error::InvalidParams)
        };
        let main_frame = get_object!(ctx, &guid, Frame)?;
        let var = Mutex::new(Variable {
            frames: vec![main_frame.clone()],
            viewport,
            ..Variable::default()
        });
        Ok(Self {
            channel,
            main_frame,
            browser_context,
            var,
            tx: Mutex::default()
        })
    }

    pub(crate) fn hook_created(&self, this: Weak<Page>) -> Result<(), Error> {
        upgrade(&self.main_frame)?.set_page(this);
        Ok(())
    }

    pub(crate) fn browser_context(&self) -> Weak<BrowserContext> { self.browser_context.clone() }

    pub(crate) fn main_frame(&self) -> Weak<Frame> { self.main_frame.clone() }

    navigation! {reload, "reload"}
    navigation! {go_back, "goBack"}
    navigation! {go_forward, "goForward"}

    pub(crate) async fn key_down(&self, key: &str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("key", key);
        let _ = send_message!(self, "keyboardDown", args);
        Ok(())
    }

    pub(crate) async fn key_up(&self, key: &str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("key", key);
        let _ = send_message!(self, "keyboardUp", args);
        Ok(())
    }

    pub(crate) async fn key_input_text(&self, text: &str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("text", text);
        let _ = send_message!(self, "keyboardInsertText", args);
        Ok(())
    }

    pub(crate) async fn key_type(&self, text: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'b> {
            text: &'b str,
            delay: Option<f64>
        }
        let args = Args { text, delay };
        let _ = send_message!(self, "keyboardInsertText", args);
        Ok(())
    }

    pub(crate) async fn key_press(&self, text: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'b> {
            text: &'b str,
            delay: Option<f64>
        }
        let args = Args { text, delay };
        let _ = send_message!(self, "keyboardPress", args);
        Ok(())
    }

    pub(crate) async fn screen_tap(&self, x: f64, y: f64) -> Result<(), Arc<Error>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            x: f64,
            y: f64
        }
        let args = Args { x, y };
        let _ = send_message!(self, "touchscreenTap", args);
        Ok(())
    }

    pub(crate) async fn mouse_move(
        &self,
        x: f64,
        y: f64,
        steps: Option<i32>
    ) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            x: f64,
            y: f64,
            steps: Option<i32>
        }
        let args = Args { x, y, steps };
        let _ = send_message!(self, "mouseMove", args);
        Ok(())
    }

    mouse_down! {mouse_down, "mouseDown"}
    mouse_down! {mouse_up, "mouseUp"}

    pub(crate) async fn mouse_click(&self, args: MouseClickArgs) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "mouseClick", args);
        Ok(())
    }

    pub(crate) async fn mouse_dblclick(&self, args: MouseClickArgs) -> Result<(), Arc<Error>> {
        let args = MouseClickArgs {
            click_count: Some(2),
            ..args
        };
        self.mouse_click(args).await
    }

    pub(crate) async fn accessibility_snapshot(
        &self,
        args: AccessibilitySnapshotArgs
    ) -> ArcResult<Option<AccessibilitySnapshotResponse>> {
        let v = send_message!(self, "accessibilitySnapshot", args);
        let first = match first(&v) {
            None => return Ok(None),
            Some(x) => x
        };
        let res: AccessibilitySnapshotResponse =
            serde_json::from_value((*first).clone()).map_err(Error::Serde)?;
        Ok(Some(res))
    }

    pub(crate) async fn bring_to_front(&self) -> ArcResult<()> {
        let _ = send_message!(self, "bringToFront", Map::new());
        Ok(())
    }

    pub(crate) async fn add_init_script(&self, source: &str) -> ArcResult<()> {
        let mut args = HashMap::new();
        args.insert("source", source);
        let _ = send_message!(self, "addInitScript", args);
        Ok(())
    }

    pub(crate) async fn pdf(
        &self,
        args: PdfArgs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_>
    ) -> ArcResult<Vec<u8>> {
        let path = args.path.clone();
        let v = send_message!(self, "pdf", args);
        let b64 = only_str(&v)?;
        let bytes = base64::decode(b64).map_err(Error::InvalidBase64)?;
        may_save(path.as_deref(), &bytes)?;
        Ok(bytes)
    }

    pub(crate) async fn close(&self, run_before_unload: Option<bool>) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            run_before_unload: Option<bool>
        }
        let args = Args { run_before_unload };
        let _ = send_message!(self, "close", args);
        Ok(())
    }

    pub(crate) async fn screenshot(&self, args: ScreenshotArgs) -> ArcResult<Vec<u8>> {
        let path = args.path.clone();
        let v = send_message!(self, "screenshot", args);
        let b64 = only_str(&v)?;
        let bytes = base64::decode(b64).map_err(Error::InvalidBase64)?;
        may_save(path.as_deref(), &bytes)?;
        Ok(bytes)
    }

    pub(crate) async fn emulate_media(&self, args: EmulateMediaArgs) -> ArcResult<()> {
        let _ = send_message!(self, "emulateMedia", args);
        Ok(())
    }

    pub(crate) async fn opener(&self) -> ArcResult<Option<Weak<Page>>> {
        let v = send_message!(self, "opener", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let p = get_object!(self.context()?.lock().unwrap(), guid, Page)?;
        Ok(Some(p))
    }

    pub(crate) async fn set_extra_http_headers<T>(&self, headers: T) -> ArcResult<()>
    where
        T: IntoIterator<Item = (String, String)>
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            headers: Vec<Header>
        }
        let args = Args {
            headers: headers.into_iter().map(Header::from).collect()
        };
        let _ = send_message!(self, "setExtraHTTPHeaders", args);
        Ok(())
    }
}

fn may_save(path: Option<&Path>, bytes: &[u8]) -> Result<(), Error> {
    let path = match path {
        Some(path) => path,
        None => return Ok(())
    };
    use std::io::Write;
    let mut file = std::fs::File::create(path).map_err(Error::from)?;
    file.write(bytes).map_err(Error::from)?;
    Ok(())
}

// mutable
impl Page {
    pub(crate) fn viewport_size(&self) -> Option<Viewport> {
        self.var.lock().unwrap().viewport.clone()
    }

    pub(crate) async fn set_viewport_size(&self, viewport_size: Viewport) -> ArcResult<()> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            viewport_size: Viewport
        }
        let args = Args {
            viewport_size: viewport_size.clone()
        };
        let _ = send_message!(self, "setViewportSize", args);
        self.var.lock().unwrap().viewport = Some(viewport_size);
        Ok(())
    }

    pub(crate) fn frames(&self) -> Vec<Weak<Frame>> { self.var.lock().unwrap().frames.clone() }

    pub(crate) fn default_timeout(&self) -> u32 {
        let this = self.var.lock().unwrap().timeout;
        let parent = || {
            self.browser_context
                .upgrade()
                .map(|c| c.default_timeout())
                .unwrap_or(Self::DEFAULT_TIMEOUT)
        };
        this.unwrap_or_else(parent)
    }

    pub(crate) fn default_navigation_timeout(&self) -> u32 {
        let this = self.var.lock().unwrap().navigation_timeout;
        let parent = || {
            self.browser_context
                .upgrade()
                .map(|c| c.default_navigation_timeout())
                .unwrap_or(Self::DEFAULT_TIMEOUT)
        };
        this.unwrap_or_else(parent)
    }

    pub(crate) async fn set_default_timeout(&self, timeout: u32) -> ArcResult<()> {
        let mut args = Map::new();
        args.insert("timeout".into(), timeout.into());
        let _ = send_message!(self, "setDefaultTimeoutNoReply", args);
        self.var.lock().unwrap().timeout = Some(timeout);
        Ok(())
    }

    pub(crate) async fn set_default_navigation_timeout(&self, timeout: u32) -> ArcResult<()> {
        let mut args = Map::new();
        args.insert("timeout".into(), timeout.into());
        let _ = send_message!(self, "setDefaultNavigationTimeoutNoReply", args);
        self.var.lock().unwrap().navigation_timeout = Some(timeout);
        Ok(())
    }

    pub(crate) fn on_frame_navigated(&self, f: Weak<Frame>) {
        self.emit_event(Evt::FrameNavigated(f));
    }


    pub(crate) fn on_request(&self, request: Weak<Request>) -> Result<(), Error>{
        self.emit_event(Evt::Request(request));
        Ok(())
    }

    pub(crate) fn on_response(&self, response: Weak<Response>) -> Result<(), Error>{
        self.emit_event(Evt::Response(response));
        Ok(())
    }

    pub(crate) fn on_page_load(&self) {
        self.emit_event(Evt::Load);
    }

    pub(crate) fn on_dom_content_loaded(&self) {
        self.emit_event(Evt::DomContentLoaded);
    }

    pub(crate) fn set_video(&self, video: Video) -> Result<(), Error> {
        self.var.lock().unwrap().video = Some(video);
        Ok(())
    }

    pub(crate) fn video(&self) -> Option<Video> { self.var.lock().unwrap().video.clone() }

    fn on_close(&self, ctx: &Context) -> Result<(), Error> {
        let bc = match self.browser_context().upgrade() {
            None => return Ok(()),
            Some(b) => b
        };
        let this = get_object!(ctx, self.guid(), Page)?;
        bc.remove_page(&this);
        self.emit_event(Evt::Close);
        Ok(())
    }

    fn on_frame_attached(&self, ctx: &Context, guid: Str<Guid>) -> Result<(), Error> {
        let this = get_object!(ctx, self.guid(), Page)?;
        let f = get_object!(ctx, &guid, Frame)?;
        upgrade(&f)?.set_page(this);
        self.var.lock().unwrap().frames.push(f.clone());
        self.emit_event(Evt::FrameAttached(f));
        Ok(())
    }

    fn on_frame_detached(&self, ctx: &Context, guid: Str<Guid>) -> Result<(), Error> {
        let frames = &mut self.var.lock().unwrap().frames;
        *frames = frames
            .iter()
            .filter(|w| w.upgrade().map(|a| a.guid() != guid).unwrap_or(false))
            .cloned()
            .collect();
        let f = get_object!(ctx, &guid, Frame)?;
        self.emit_event(Evt::FrameDetached(f));
        Ok(())
    }

    fn on_request_failed(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct De {
            request: OnlyGuid,
            response_end_timing: f64,
            failure_text: Option<String>
        }
        let De {
            request: OnlyGuid { guid },
            response_end_timing,
            failure_text
        } = serde_json::from_value(params.into())?;
        let request = get_object!(ctx, &guid, Request)?;
        let req = upgrade(&request)?;
        req.set_failure(failure_text);
        req.set_response_end(response_end_timing);
        self.emit_event(Evt::RequestFailed(request));
        Ok(())
    }

    fn on_request_finished(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct De {
            request: OnlyGuid,
            response_end_timing: f64
        }
        let De {
            request: OnlyGuid { guid },
            response_end_timing
        } = serde_json::from_value(params.into())?;
        let request = get_object!(ctx, &guid, Request)?;
        let req = upgrade(&request)?;
        req.set_response_end(response_end_timing);
        self.emit_event(Evt::RequestFinished(request));
        Ok(())
    }

    pub(crate) fn workers(&self) -> Vec<Weak<Worker>> { self.var.lock().unwrap().workers.clone() }

    fn push_worker(&self, worker: Weak<Worker>) { self.var.lock().unwrap().workers.push(worker); }

    pub(crate) fn remove_worker(&self, worker: &Weak<Worker>) {
        let workers = &mut self.var.lock().unwrap().workers;
        workers.remove_one(|w| w.ptr_eq(worker));
    }

    fn on_worker(&self, ctx: &Context, worker: Weak<Worker>) -> Result<(), Error> {
        self.push_worker(worker.clone());
        let this = get_object!(ctx, self.guid(), Page)?;
        upgrade(&worker)?.set_page(this);
        self.emit_event(Evt::Worker(worker));
        Ok(())
    }

    fn on_download(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct De {
            url: String,
            suggested_filename: String,
            artifact: OnlyGuid
        }
        let De {
            url,
            suggested_filename,
            artifact: OnlyGuid { guid }
        } = serde_json::from_value(params.into())?;
        let artifact = get_object!(ctx, &guid, Artifact)?;
        // TODO: set_is_remote
        // artifactObject._isRemote = !!this._browserContext._browser && this._browserContext._browser._isRemote;
        let download = Download::new(artifact, url, suggested_filename);
        self.emit_event(Evt::Download(Arc::new(download)));
        Ok(())
    }

    fn on_video(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        let v = params.into();
        let guid = only_guid(&v)?;
        let artifact = get_object!(ctx, guid, Artifact)?;
        let video = Video::new(artifact);
        self.set_video(video.clone())?;
        self.emit_event(Evt::Video(video));
        Ok(())
    }

    fn on_file_chooser(&self, ctx: &Context, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct De {
            element: OnlyGuid,
            is_multiple: bool
        }
        let De {
            element: OnlyGuid { guid },
            is_multiple
        } = serde_json::from_value(params.into())?;
        let element = get_object!(ctx, &guid, ElementHandle)?;
        let this = get_object!(ctx, self.guid(), Page)?;
        let file_chooser = FileChooser::new(this, element, is_multiple);
        // self.emit_event(Evt::FileChooser(file_chooser));
        Ok(())
    }
}

impl RemoteObject for Page {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        ctx: &Context,
        method: Str<Method>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        dbg!(&method);
        match method.as_str() {
            "close" => self.on_close(ctx)?,
            "frameattached" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                self.on_frame_attached(ctx, guid)?;
            }
            "framedetached" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                self.on_frame_detached(ctx, guid)?;
            }
            "load" => self.emit_event(Evt::Load),
            "domcontentloaded" => self.emit_event(Evt::DomContentLoaded),
            "crash" => self.emit_event(Evt::Crash),
            "console" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let console = get_object!(ctx, &guid, ConsoleMessage)?;
                self.emit_event(Evt::Console(console));
            }
            "request" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let request = get_object!(ctx, &guid, Request)?;
                self.emit_event(Evt::Request(request));
            }
            "requestfailed" => self.on_request_failed(ctx, params)?,
            "requestfinished" => self.on_request_finished(ctx, params)?,
            "response" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let response = get_object!(ctx, &guid, Response)?;
                self.emit_event(Evt::Response(response));
            }
            "popup" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let page = get_object!(ctx, &guid, Page)?;
                self.emit_event(Evt::Popup(page));
            }
            "websocket" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let websocket = get_object!(ctx, &guid, WebSocket)?;
                self.emit_event(Evt::WebSocket(websocket));
            }
            "worker" => {
                let first = first_object(&params).ok_or(Error::InvalidParams)?;
                let OnlyGuid { guid } = serde_json::from_value((*first).clone())?;
                let worker = get_object!(ctx, &guid, Worker)?;
                self.on_worker(ctx, worker)?;
            }
            "download" => self.on_download(ctx, params)?,
            "video" => self.on_video(ctx, params)?,
            "filechooser" => self.on_file_chooser(ctx, params)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Evt {
    Close,
    Crash,
    Console(Weak<ConsoleMessage>),
    /// Not Implemented Yet
    Dialog,
    Download(Arc<Download>),
    /// Not Implemented Yet
    // FileChooser(FileChooser),
    DomContentLoaded,
    /// Not Implemented Yet
    PageError,
    Request(Weak<Request>),
    Response(Weak<Response>),
    RequestFailed(Weak<Request>),
    RequestFinished(Weak<Request>),
    FrameAttached(Weak<Frame>),
    FrameDetached(Weak<Frame>),
    FrameNavigated(Weak<Frame>),
    Load,
    Popup(Weak<Page>),
    WebSocket(Weak<WebSocket>),
    Worker(Weak<Worker>),
    Video(Video)
}

impl EventEmitter for Page {
    type Event = Evt;
    fn tx(&self) -> Option<broadcast::Sender<Self::Event>> { self.tx.lock().unwrap().clone() }
    fn set_tx(&self, tx: broadcast::Sender<Self::Event>) { *self.tx.lock().unwrap() = Some(tx); }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Close,
    Crash,
    Console,
    Dialog,
    Download,
    // FileChooser,
    DomContentLoaded,
    PageError,
    Request,
    Response,
    RequestFailed,
    RequestFinished,
    FrameAttached,
    FrameDetached,
    FrameNavigated,
    Load,
    Popup,
    WebSocket,
    Worker,
    Video
}

impl IsEvent for Evt {
    type EventType = EventType;

    fn event_type(&self) -> Self::EventType {
        match self {
            Self::Close => EventType::Close,
            Self::Crash => EventType::Crash,
            Self::Console(_) => EventType::Console,
            Self::Dialog => EventType::Dialog,
            Self::Download(_) => EventType::Download,
            // Self::FileChooser(_) => EventType::FileChooser,
            Self::DomContentLoaded => EventType::DomContentLoaded,
            Self::PageError => EventType::PageError,
            Self::Request(_) => EventType::Request,
            Self::Response(_) => EventType::Response,
            Self::RequestFailed(_) => EventType::RequestFailed,
            Self::RequestFinished(_) => EventType::RequestFinished,
            Self::FrameAttached(_) => EventType::FrameAttached,
            Self::FrameDetached(_) => EventType::FrameDetached,
            Self::FrameNavigated(_) => EventType::FrameNavigated,
            Self::Load => EventType::Load,
            Self::Popup(_) => EventType::Popup,
            Self::WebSocket(_) => EventType::WebSocket,
            Self::Worker(_) => EventType::Worker,
            Self::Video(_) => EventType::Video
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    main_frame: OnlyGuid,
    #[serde(rename = "viewportSize")]
    viewport: Option<Viewport>
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MouseClickArgs {
    x: f64,
    y: f64,
    pub(crate) delay: Option<f64>,
    pub(crate) button: Option<MouseButton>,
    pub(crate) click_count: Option<i32>
}

impl MouseClickArgs {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            delay: None,
            button: None,
            click_count: None
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccessibilitySnapshotArgs {
    pub(crate) interesting_only: Option<bool>,
    pub(crate) root: Option<OnlyGuid>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilitySnapshotResponse {
    pub role: String,
    pub name: String,
    pub value: Option<Val>,
    pub description: Option<String>,
    pub keyshortcuts: Option<String>,
    pub roledescription: Option<String>,
    pub valuetext: Option<String>,
    pub disabled: Option<bool>,
    pub expanded: Option<bool>,
    pub focused: Option<bool>,
    pub modal: Option<bool>,
    pub multiline: Option<bool>,
    pub multiselectable: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub selected: Option<bool>,
    pub checked: Option<Mixed>,
    pub pressed: Option<Mixed>,
    pub level: Option<i64>,
    pub valuemin: Option<f64>,
    pub valuemax: Option<f64>,
    pub autocomplete: Option<String>,
    pub haspopup: Option<String>,
    pub invalid: Option<String>,
    pub orientation: Option<String>,
    #[serde(default)]
    pub children: Vec<AccessibilitySnapshotResponse>
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Val {
    String(String),
    Number(f64)
}
#[derive(Debug, Deserialize, PartialEq)]
pub enum Mixed {
    Mixed,
    Bool(bool)
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReloadArgs {
    pub(crate) timeout: Option<f64>,
    pub(crate) wait_until: Option<LifecycleEvent>
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PdfArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    pub(crate) scale: Option<f64>,
    pub(crate) display_header_footer: Option<bool>,
    pub(crate) header_template: Option<&'a str>,
    pub(crate) footer_template: Option<&'b str>,
    pub(crate) print_background: Option<bool>,
    pub(crate) landscape: Option<bool>,
    pub(crate) page_ranges: Option<&'c str>,
    pub(crate) format: Option<&'d str>,
    pub(crate) width: Option<Length<'e>>,
    pub(crate) height: Option<Length<'f>>,
    #[serde(rename = "preferCSSPageSize")]
    pub(crate) prefer_css_page_size: Option<bool>,
    pub(crate) margin: Option<PdfMargins<'g, 'h, 'i, 'j>>,
    pub(crate) path: Option<PathBuf>
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScreenshotArgs {
    pub(crate) timeout: Option<f64>,
    pub(crate) r#type: Option<ScreenshotType>,
    pub(crate) quality: Option<i32>,
    pub(crate) omit_background: Option<bool>,
    pub(crate) full_page: Option<bool>,
    pub(crate) clip: Option<FloatRect>,
    pub(crate) path: Option<PathBuf>
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmulateMediaArgs {
    pub(crate) media: Option<Media>,
    pub(crate) color_scheme: Option<ColorScheme>
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Media {
    /// Reset emulating
    Null,
    Print,
    Screen
}
