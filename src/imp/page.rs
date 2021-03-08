use crate::imp::{
    core::*,
    frame::{ClickArgs, Frame, GotoArgs},
    prelude::*,
    response::Response,
    utils::{DocumentLoadState, MouseButton, Viewport}
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
}

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
