use crate::imp::{
    core::*,
    frame::Frame,
    prelude::*,
    response::Response,
    utils::{
        ColorScheme, DocumentLoadState, FloatRect, Length, MouseButton, PdfMargins, ScreenshotType,
        Viewport
    }
};

#[derive(Debug)]
pub(crate) struct Page {
    channel: ChannelOwner,
    viewport: Option<Viewport>,
    main_frame: Weak<Frame>
}

#[derive(Debug)]
pub(crate) struct BindingCall {
    channel: ChannelOwner
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
            let r = find_object!(self.context()?.lock().unwrap(), &guid, Response)?;
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
            #[derive(Serialize)]
            struct Args {
                #[serde(skip_serializing_if = "Option::is_none")]
                button: Option<MouseButton>,
                #[serde(skip_serializing_if = "Option::is_none")]
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
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            main_frame: OnlyGuid { guid },
            viewport
        } = serde_json::from_value(channel.initializer.clone())?;
        let main_frame = find_object!(ctx, &guid, Frame)?;
        Ok(Self {
            channel,
            viewport,
            main_frame
        })
    }

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
        #[derive(Serialize)]
        struct Args<'b> {
            text: &'b str,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay: Option<f64>
        }
        let args = Args { text, delay };
        let _ = send_message!(self, "keyboardInsertText", args);
        Ok(())
    }

    pub(crate) async fn key_press(&self, text: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        #[derive(Serialize)]
        struct Args<'b> {
            text: &'b str,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay: Option<f64>
        }
        let args = Args { text, delay };
        let _ = send_message!(self, "keyboardPress", args);
        Ok(())
    }

    pub(crate) async fn screen_tap(&self, x: f64, y: f64) -> Result<(), Arc<Error>> {
        #[derive(Serialize)]
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
        #[derive(Serialize)]
        struct Args {
            x: f64,
            y: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
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

    // pub(crate) async fn accessibility_snapshot(&self, args: AccessibilitySnapshoptArgs) {}

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
        let v = send_message!(self, "pdf", args);
        let b64 = only_str(&v)?;
        let bytes = base64::decode(b64).map_err(Error::InvalidBase64)?;
        Ok(bytes)
    }

    pub(crate) async fn close(&self, run_before_unload: Option<bool>) -> Result<(), Arc<Error>> {
        #[derive(Serialize)]
        struct Args {
            #[serde(skip_serializing_if = "Option::is_none")]
            run_before_unload: Option<bool>
        }
        let args = Args { run_before_unload };
        let _ = send_message!(self, "close", args);
        Ok(())
    }

    pub(crate) async fn screenshot(&self, args: ScreenshotArgs) -> ArcResult<Vec<u8>> {
        let v = send_message!(self, "screenshot", args);
        let b64 = only_str(&v)?;
        let bytes = base64::decode(b64).map_err(Error::InvalidBase64)?;
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
        let p = find_object!(self.context()?.lock().unwrap(), &guid, Page)?;
        Ok(Some(p))
    }
}

// mutable
impl Page {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MouseClickArgs {
    x: f64,
    y: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) button: Option<MouseButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccessibilitySnapshoptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    interesting_only: Option<bool> // root: Option<Arc<ElementHandle>>
}

impl BindingCall {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }
}

impl RemoteObject for Page {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

impl RemoteObject for BindingCall {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    main_frame: OnlyGuid,
    #[serde(rename = "viewportSIze")]
    viewport: Option<Viewport>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReloadArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wait_until: Option<DocumentLoadState>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PdfArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display_header_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) header_template: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) footer_template: Option<&'b str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) print_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) landscape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_ranges: Option<&'c str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<&'d str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) width: Option<Length<'e>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) height: Option<Length<'f>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preferCSSPageSize")]
    pub(crate) prefer_css_page_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) margin: Option<PdfMargins<'g, 'h, 'i, 'j>>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScreenshotArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) r#type: Option<ScreenshotType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) quality: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) omit_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) full_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip: Option<FloatRect>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EmulateMediaArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color_scheme: Option<ColorScheme>
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Media {
    Print,
    Screen
}
