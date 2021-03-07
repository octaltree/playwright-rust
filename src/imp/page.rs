use crate::imp::{
    core::*,
    frame::{ClickArgs, Frame, GotoArgs},
    prelude::*,
    response::Response,
    utils::{DocumentLoadState, Viewport}
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

    pub(crate) async fn key_down<'b>(&self, key: &'b str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("key", key);
        let _ = send_message!(self, "keyboardDown", args);
        Ok(())
    }

    pub(crate) async fn key_up<'b>(&self, key: &'b str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("key", key);
        let _ = send_message!(self, "keyboardUp", args);
        Ok(())
    }

    pub(crate) async fn key_input_text<'b>(&self, text: &'b str) -> Result<(), Arc<Error>> {
        let mut args = HashMap::new();
        args.insert("text", text);
        let _ = send_message!(self, "keyboardInsertText", args);
        Ok(())
    }

    pub(crate) async fn key_type<'b>(
        &self,
        text: &'b str,
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
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

    pub(crate) async fn key_press<'b>(
        &self,
        text: &'b str,
        delay: Option<f64>
    ) -> Result<(), Arc<Error>> {
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
