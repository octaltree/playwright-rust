use crate::{
    imp::{
        browser_context::BrowserContext,
        core::*,
        frame::Frame,
        prelude::*,
        utils::{
            ElementState, File, FloatRect, KeyboardModifier, MouseButton, Position, ScreenshotType,
            WaitForSelectorState,
        },
    },
    protocol::generated::WritableStream,
};
use itertools::Itertools;
use std::fs;

#[derive(Debug)]
pub(crate) struct ElementHandle {
    channel: ChannelOwner,
}

macro_rules! is_checked {
    ($f: ident, $m: literal) => {
        pub(crate) async fn $f(&self) -> ArcResult<bool> {
            let v = send_message!(self, $m, Map::new());
            let b = first(&v)
                .ok_or(Error::InvalidParams)?
                .as_bool()
                .ok_or(Error::InvalidParams)?;
            Ok(b)
        }
    };
}

impl ElementHandle {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

    pub(crate) async fn query_selector(
        &self,
        selector: &str,
    ) -> ArcResult<Option<Weak<ElementHandle>>> {
        let mut args = HashMap::new();
        args.insert("selector", selector);
        let v = send_message!(self, "querySelector", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let e = get_object!(self.context()?.lock(), guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn query_selector_all(
        &self,
        selector: &str,
    ) -> ArcResult<Vec<Weak<ElementHandle>>> {
        let mut args = HashMap::new();
        args.insert("selector", selector);
        let v = send_message!(self, "querySelectorAll", args);
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let elements: Vec<OnlyGuid> =
            serde_json::from_value((*first).clone()).map_err(Error::Serde)?;
        let es = elements
            .into_iter()
            .map(|OnlyGuid { guid }| {
                get_object!(self.context()?.lock(), &guid, ElementHandle)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(es)
    }

    pub(crate) async fn inner_text(&self) -> ArcResult<String> {
        let v = send_message!(self, "innerText", Map::new());
        let s = only_str(&v)?;
        Ok(s.to_owned())
    }

    pub(crate) async fn inner_html(&self) -> ArcResult<String> {
        let v = send_message!(self, "innerHTML", Map::new());
        let s = only_str(&v)?;
        Ok(s.to_owned())
    }

    is_checked! {is_checked, "isChecked"}
    is_checked! {is_disabled, "isDisabled"}
    is_checked! {is_editable, "isEditable"}
    is_checked! {is_enabled, "isEnabled"}
    is_checked! {is_hidden, "isHidden"}
    is_checked! {is_visible, "isVisible"}

    pub(crate) async fn owner_frame(&self) -> ArcResult<Option<Weak<Frame>>> {
        let v = send_message!(self, "ownerFrame", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let f = get_object!(self.context()?.lock(), guid, Frame)?;
        Ok(Some(f))
    }

    pub(crate) async fn content_frame(&self) -> ArcResult<Option<Weak<Frame>>> {
        let v = send_message!(self, "contentFrame", Map::new());
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let f = get_object!(self.context()?.lock(), guid, Frame)?;
        Ok(Some(f))
    }

    pub(crate) async fn get_attribute(&self, name: &str) -> ArcResult<Option<String>> {
        let mut args = HashMap::new();
        args.insert("name", name);
        let v = send_message!(self, "getAttribute", args);
        let s = maybe_only_str(&v)?;
        Ok(s.map(ToOwned::to_owned))
    }

    pub(crate) async fn text_content(&self) -> ArcResult<Option<String>> {
        let v = send_message!(self, "textContent", Map::new());
        let s = maybe_only_str(&v)?;
        Ok(s.map(ToOwned::to_owned))
    }

    pub(crate) async fn hover(&self, args: HoverArgs) -> ArcResult<()> {
        let _ = send_message!(self, "hover", args);
        Ok(())
    }

    pub(crate) async fn click(&self, args: ClickArgs) -> ArcResult<()> {
        let _ = send_message!(self, "click", args);
        Ok(())
    }

    pub(crate) async fn dblclick(&self, args: ClickArgs) -> ArcResult<()> {
        let _ = send_message!(self, "dblclick", args);
        Ok(())
    }

    pub(crate) async fn check(&self, args: CheckArgs) -> ArcResult<()> {
        let _ = send_message!(self, "check", args);
        Ok(())
    }

    pub(crate) async fn uncheck(&self, args: CheckArgs) -> ArcResult<()> {
        let _ = send_message!(self, "uncheck", args);
        Ok(())
    }

    pub(crate) async fn tap(&self, args: TapArgs) -> ArcResult<()> {
        let _ = send_message!(self, "tap", args);
        Ok(())
    }

    pub(crate) async fn fill(&self, args: FillArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "fill", args);
        Ok(())
    }

    pub(crate) async fn focus(&self) -> ArcResult<()> {
        let _ = send_message!(self, "focus", Map::new());
        Ok(())
    }

    pub(crate) async fn r#type(&self, args: TypeArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "type", args);
        Ok(())
    }

    pub(crate) async fn press(&self, args: PressArgs<'_>) -> ArcResult<()> {
        let _ = send_message!(self, "press", args);
        Ok(())
    }

    pub(crate) async fn scroll_into_view_if_needed(&self, timeout: Option<f64>) -> ArcResult<()> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let _ = send_message!(self, "scrollIntoViewIfNeeded", args);
        Ok(())
    }

    pub(crate) async fn select_text(&self, timeout: Option<f64>) -> ArcResult<()> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let _ = send_message!(self, "selectText", args);
        Ok(())
    }

    pub(crate) async fn bounding_box(&self) -> ArcResult<Option<FloatRect>> {
        let v = send_message!(self, "boundingBox", Map::new());
        let v = match first(&v) {
            None => return Ok(None),
            Some(v) => v
        };
        let f: FloatRect = serde_json::from_value((*v).clone()).map_err(Error::Serde)?;
        Ok(Some(f))
    }

    pub(crate) async fn screenshot(&self, args: ScreenshotArgs<'_>) -> ArcResult<Vec<u8>> {
        let path = args.path.clone();
        let v = send_message!(self, "screenshot", args);
        let b64 = only_str(&v)?;
        let bytes = base64::decode(b64).map_err(Error::InvalidBase64)?;
        may_save(path.as_deref(), &bytes)?;
        Ok(bytes)
    }

    pub(crate) async fn wait_for_element_state(
        &self,
        state: ElementState,
        timeout: Option<f64>,
    ) -> ArcResult<()> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            state: ElementState,
            timeout: Option<f64>,
        }
        let args = Args { state, timeout };
        let _ = send_message!(self, "waitForElementState", args);
        Ok(())
    }

    pub(crate) async fn wait_for_selector(
        &self,
        args: WaitForSelectorArgs<'_>,
    ) -> ArcResult<Option<Weak<ElementHandle>>> {
        let v = send_message!(self, "waitForSelector", args);
        let guid = match as_only_guid(&v) {
            Some(g) => g,
            None => return Ok(None)
        };
        let e = get_object!(self.context()?.lock(), guid, ElementHandle)?;
        Ok(Some(e))
    }

    pub(crate) async fn dispatch_event<T>(
        &self,
        r#type: &str,
        event_init: Option<T>,
    ) -> ArcResult<()>
        where
            T: Serialize
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            r#type: &'a str,
            event_init: Value,
        }
        let event_init = ser::to_value(&event_init).map_err(Error::SerializationPwJson)?;
        let args = Args { r#type, event_init };
        let _ = send_message!(self, "dispatchEvent", args);
        Ok(())
    }

    pub(crate) async fn select_option(&self, args: SelectOptionArgs) -> ArcResult<Vec<String>> {
        let v = send_message!(self, "selectOption", args);
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let ss = first
            .as_array()
            .ok_or(Error::InvalidParams)?
            .iter()
            .filter_map(|v| v.as_str())
            .map(ToOwned::to_owned)
            .collect();
        Ok(ss)
    }

    pub(crate) async fn set_input_files(&self, args: SetInputFilesArgs) -> ArcResult<()> {
        let _ = send_message!(self, "setInputFiles", args);
        Ok(())
    }

    async fn browser_context(&self) -> ArcResult<Arc<BrowserContext>> {
        Ok(self
            .owner_frame()
            .await
            .unwrap()
            .unwrap()
            .upgrade()
            .unwrap()
            .page()
            .unwrap()
            .upgrade()
            .unwrap()
            .browser_context()
            .upgrade()
            .unwrap())
    }

    /// # stages:
    /// * create temp files using browser
    /// * create streams
    /// * assign each files to a stream
    /// * send setInputFilesPaths message
    pub(crate) async fn set_input_file_paths(&self, args: SetInputFilePathsArgs) -> ArcResult<()> {
        let browser_context = self.browser_context().await?;
        if browser_context.browser().is_some() && browser_context
            .browser()
            .unwrap()
            .upgrade()
            .unwrap()
            .is_remote() {
            panic!("Not implemented yet");
            // for local_path in &args.local_paths {
            //     let guid = browser_context.create_temp_file(local_path).await?;
            //     args.streams.as_mut().unwrap().push(guid);
            //     let f = fs::File::open(local_path).unwrap();
            //     guid.
            // }
        }
        send_message!(self, "setInputFilePaths", args);
        Ok(())
    }
}

pub(super) fn may_save(path: Option<&Path>, bytes: &[u8]) -> Result<(), Error> {
    let path = match path {
        Some(path) => path,
        None => return Ok(())
    };
    use std::io::Write;
    let mut file = std::fs::File::create(path).map_err(Error::from)?;
    file.write(bytes).map_err(Error::from)?;
    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HoverArgs {
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    pub(crate) position: Option<Position>,
    pub(crate) timeout: Option<f64>,
    pub(crate) force: Option<bool>,
    pub(crate) trial: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClickArgs {
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    pub(crate) position: Option<Position>,
    pub(crate) delay: Option<f64>,
    pub(crate) button: Option<MouseButton>,
    /// Is ignored if dblclick
    pub(crate) click_count: Option<i32>,
    pub(crate) timeout: Option<f64>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) trial: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckArgs {
    pub(crate) position: Option<Position>,
    pub(crate) timeout: Option<f64>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) trial: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TapArgs {
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    pub(crate) position: Option<Position>,
    pub(crate) timeout: Option<f64>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) trial: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FillArgs<'a> {
    value: &'a str,
    pub(crate) timeout: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
}

impl<'a> FillArgs<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        Self {
            value,
            timeout: None,
            no_wait_after: None,
        }
    }
}

macro_rules! type_args {
    ($t:ident, $f:ident) => {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub(crate) struct $t<'a> {
            $f: &'a str,
            pub(crate) delay: Option<f64>,
            pub(crate) timeout: Option<f64>,
            pub(crate) no_wait_after: Option<bool>
        }

        impl<'a> $t<'a> {
            pub(crate) fn new($f: &'a str) -> Self {
                Self {
                    $f,
                    delay: None,
                    timeout: None,
                    no_wait_after: None
                }
            }
        }
    };
}

type_args! {TypeArgs, text}
type_args! {PressArgs, key}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScreenshotArgs<'a> {
    pub(crate) path: Option<&'a Path>,
    pub(crate) timeout: Option<f64>,
    pub(crate) r#type: Option<ScreenshotType>,
    pub(crate) quality: Option<i64>,
    pub(crate) omit_background: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WaitForSelectorArgs<'a> {
    selector: &'a str,
    pub(crate) state: Option<WaitForSelectorState>,
    pub(crate) timeout: Option<f64>,
}

impl<'a> WaitForSelectorArgs<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self {
            selector,
            state: None,
            timeout: None,
        }
    }
}

impl RemoteObject for ElementHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SelectOptionArgs {
    pub(crate) options: Option<Vec<Opt>>,
    pub(crate) elements: Option<Vec<OnlyGuid>>,

    pub(crate) timeout: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
}

#[derive(Serialize)]
pub(crate) enum Opt {
    Value(String),
    Index(usize),
    Label(String),
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetInputFilesArgs {
    pub(crate) files: Vec<File>,
    pub(crate) timeout: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetInputFilePathsArgs {
    pub(crate) local_paths: Option<Vec<PathBuf>>,
    pub(crate) stream: Option<Vec<WritableStream>>,
    pub(crate) timeout: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
}
