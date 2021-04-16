use crate::{
    api::Frame,
    imp::{
        core::*,
        element_handle::{
            CheckArgs, ClickArgs, ElementHandle as Impl, FillArgs, HoverArgs, Opt, PressArgs,
            ScreenshotArgs, SelectOptionArgs, SetInputFilesArgs, TapArgs, TypeArgs,
            WaitForSelectorArgs
        },
        prelude::*,
        utils::{
            ElementState, File, FloatRect, KeyboardModifier, MouseButton, Position, ScreenshotType
        }
    }
};

/// ElementHandle represents an in-page DOM element. ElementHandles can be created with the [`method: Page.querySelector`]
/// method.
///
/// ```js
/// const { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.
///
/// (async () => {
///  const browser = await chromium.launch();
///  const page = await browser.newPage();
///  await page.goto('https://example.com');
///  const hrefElement = await page.$('a');
///  await hrefElement.click();
///  // ...
/// })();
/// ```
///
/// ElementHandle prevents DOM element from garbage collection unless the handle is disposed with
/// [`method: JSHandle.dispose`]. ElementHandles are auto-disposed when their origin frame gets navigated.
///
/// ElementHandle instances can be used as an argument in [`method: Page.evalOnSelector`] and [`method: Page.evaluate`]
/// methods.
pub struct ElementHandle {
    inner: Weak<Impl>
}

impl PartialEq for ElementHandle {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

macro_rules! is_checked {
    ($f: ident) => {
        pub async fn $f(&self) -> ArcResult<bool> { upgrade(&self.inner)?.$f().await }
    };
}

impl ElementHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub(crate) fn guid(&self) -> Result<Str<Guid>, Error> {
        Ok(upgrade(&self.inner)?.guid().to_owned())
    }

    pub async fn query_selector(&self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        Ok(upgrade(&self.inner)?
            .query_selector(selector)
            .await?
            .map(ElementHandle::new))
    }

    pub async fn query_selector_all(&self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        let es = upgrade(&self.inner)?.query_selector_all(selector).await?;
        Ok(es.into_iter().map(ElementHandle::new).collect())
    }

    pub async fn inner_text(&self) -> ArcResult<String> { upgrade(&self.inner)?.inner_text().await }

    pub async fn inner_html(&self) -> ArcResult<String> { upgrade(&self.inner)?.inner_html().await }

    is_checked! {is_checked}
    is_checked! {is_disabled}
    is_checked! {is_editable}
    is_checked! {is_enabled}
    is_checked! {is_hidden}
    is_checked! {is_visible}

    pub async fn owner_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.owner_frame().await?.map(Frame::new))
    }

    pub async fn content_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.content_frame().await?.map(Frame::new))
    }

    pub async fn get_attribute(&self, name: &str) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.get_attribute(name).await
    }

    pub async fn text_content(&self) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.text_content().await
    }

    pub fn hover_builder(&self) -> HoverBuilder { HoverBuilder::new(self.inner.clone()) }

    pub fn click_builder(&self) -> ClickBuilder { ClickBuilder::new(self.inner.clone()) }

    pub fn dblclick_builder(&self) -> DblClickBuilder { DblClickBuilder::new(self.inner.clone()) }

    pub fn check_builder(&self) -> CheckBuilder { CheckBuilder::new(self.inner.clone()) }

    pub fn uncheck_builder(&self) -> UncheckBuilder { UncheckBuilder::new(self.inner.clone()) }

    pub fn tap_builder(&self) -> TapBuilder { TapBuilder::new(self.inner.clone()) }

    pub fn fill_builder<'a>(&self, value: &'a str) -> FillBuilder<'a> {
        FillBuilder::new(self.inner.clone(), value)
    }

    pub async fn focus(&self) -> ArcResult<()> { upgrade(&self.inner)?.focus().await }

    pub fn type_builder<'a>(&self, text: &'a str) -> TypeBuilder<'a> {
        TypeBuilder::new(self.inner.clone(), text)
    }

    pub fn press_builder<'a>(&self, key: &'a str) -> PressBuilder<'a> {
        PressBuilder::new(self.inner.clone(), key)
    }

    pub async fn scroll_into_view_if_needed(&self, timeout: Option<f64>) -> ArcResult<()> {
        upgrade(&self.inner)?
            .scroll_into_view_if_needed(timeout)
            .await
    }

    pub async fn select_text(&self, timeout: Option<f64>) -> ArcResult<()> {
        upgrade(&self.inner)?.select_text(timeout).await
    }

    pub async fn bounding_box(&self) -> ArcResult<Option<FloatRect>> {
        upgrade(&self.inner)?.bounding_box().await
    }

    pub async fn screenshot_builder(&self) -> ScreenshotBuilder {
        ScreenshotBuilder::new(self.inner.clone())
    }

    pub async fn wait_for_element_state(
        &self,
        state: ElementState,
        timeout: Option<f64>
    ) -> ArcResult<()> {
        upgrade(&self.inner)?
            .wait_for_element_state(state, timeout)
            .await
    }

    pub fn wait_for_selector_builder<'a>(&self, selector: &'a str) -> WaitForSelectorBuilder<'a> {
        WaitForSelectorBuilder::new(self.inner.clone(), selector)
    }

    pub async fn dispatch_event<T>(&self, r#type: &str, event_init: Option<T>) -> ArcResult<()>
    where
        T: Serialize
    {
        upgrade(&self.inner)?
            .dispatch_event(r#type, event_init)
            .await
    }

    pub fn select_option_builder(&self) -> SelectOptionBuilder {
        SelectOptionBuilder::new(self.inner.clone())
    }

    pub fn set_input_files_builder(&self, file: File) -> SetInputFilesBuilder {
        SetInputFilesBuilder::new(self.inner.clone(), file)
    }
}

// TODO: JsHandle
impl ElementHandle {}

pub struct HoverBuilder {
    inner: Weak<Impl>,
    args: HoverArgs
}

impl HoverBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = HoverArgs::default();
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.hover(args).await
    }

    optional_setter!(
        modifiers, Vec<KeyboardModifier>;
        position, Position;
        timeout, f64;
        force, bool);
}

macro_rules! clicker {
    ($t: ident, $f: ident) => {
        pub struct $t {
            inner: Weak<Impl>,
            args: ClickArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<Impl>) -> Self {
                let args = ClickArgs::default();
                Self { inner, args }
            }

            pub async fn $f(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$f(args).await?;
                Ok(())
            }

            optional_setter!(
                modifiers, Vec<KeyboardModifier>;
                position, Position;
                delay, f64;
                button, MouseButton;
                click_count, i32;
                timeout, f64;
                force, bool;
                no_wait_after, bool);
        }
    }
}

clicker!(ClickBuilder, click);
clicker!(DblClickBuilder, dblclick);

macro_rules! check_builder {
    ($t: ident, $m: ident) => {
        pub struct $t {
            inner: Weak<Impl>,
            args: CheckArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<Impl>) -> Self {
                let args = CheckArgs::default();
                Self { inner, args }
            }

            pub async fn $m(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$m(args).await?;
                Ok(())
            }

            optional_setter!(
                timeout, f64;
                force, bool;
                no_wait_after, bool);
        }
    }
}

check_builder!(CheckBuilder, check);
check_builder!(UncheckBuilder, uncheck);

pub struct TapBuilder {
    inner: Weak<Impl>,
    args: TapArgs
}

impl TapBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = TapArgs::default();
        Self { inner, args }
    }

    pub async fn tap(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.tap(args).await?;
        Ok(())
    }

    optional_setter!(
        modifiers, Vec<KeyboardModifier>;
        position, Position;
        timeout, f64;
        force, bool;
        no_wait_after, bool);
}

pub struct FillBuilder<'a> {
    inner: Weak<Impl>,
    args: FillArgs<'a>
}

impl<'a> FillBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, value: &'a str) -> Self {
        let args = FillArgs::new(value);
        Self { inner, args }
    }

    pub async fn fill(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.fill(args).await?;
        Ok(())
    }

    optional_setter!(
        timeout, f64;
        no_wait_after, bool);
}

macro_rules! type_builder {
    ($t: ident, $a: ident, $f: ident, $m: ident) => {
        pub struct $t<'a> {
            inner: Weak<Impl>,
            args: $a<'a>
        }

        impl<'a> $t<'a> {
            pub(crate) fn new(inner: Weak<Impl>, $f: &'a str) -> Self {
                let args = $a::new($f);
                Self { inner, args }
            }

            pub async fn $m(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$m(args).await?;
                Ok(())
            }

            optional_setter!(
                delay, f64;
                timeout, f64;
                no_wait_after, bool);
        }
    }
}

type_builder!(TypeBuilder, TypeArgs, text, r#type);
type_builder!(PressBuilder, PressArgs, key, press);

pub struct ScreenshotBuilder {
    inner: Weak<Impl>,
    args: ScreenshotArgs
}

impl ScreenshotBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = ScreenshotArgs::default();
        Self { inner, args }
    }

    pub async fn screenshot(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.screenshot(args).await?;
        Ok(())
    }

    pub fn r#type(mut self, x: ScreenshotType) -> Self {
        self.args.r#type = Some(x);
        self
    }

    optional_setter!(
        timeout, f64;
        quality, i32;
        omit_background, bool);

    pub fn clear_type(mut self) -> Self {
        self.args.r#type = None;
        self
    }
}

pub struct WaitForSelectorBuilder<'a> {
    inner: Weak<Impl>,
    args: WaitForSelectorArgs<'a>
}

impl<'a> WaitForSelectorBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = WaitForSelectorArgs::new(selector);
        Self { inner, args }
    }

    pub async fn wait_for_selector(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        let _ = upgrade(&inner)?.wait_for_selector(args).await?;
        Ok(())
    }

    optional_setter!(
        state, ElementState;
        timeout, f64);
}

pub struct SelectOptionBuilder {
    inner: Weak<Impl>,
    args: SelectOptionArgs,
    err: Option<Error>
}

impl SelectOptionBuilder {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = SelectOptionArgs::default();
        Self {
            inner,
            args,
            err: None
        }
    }

    pub async fn select_option(self) -> Result<Vec<String>, Arc<Error>> {
        let Self { inner, args, err } = self;
        if let Some(e) = err {
            return Err(e.into());
        }
        upgrade(&inner)?.select_option(args).await
    }

    pub fn add_element(mut self, x: &ElementHandle) -> Self {
        let guid = match x.guid() {
            Ok(i) => i,
            Err(e) => {
                if self.err.is_none() {
                    self.err = Some(e);
                }
                return self;
            }
        };
        let x = OnlyGuid { guid };
        if let Some(e) = &mut self.args.elements {
            e.push(x);
        } else {
            self.args.elements = Some(vec![x]);
        }
        self
    }

    pub fn add_value(mut self, x: String) -> Self {
        let x = Opt::Value(x);
        if let Some(o) = &mut self.args.options {
            o.push(x);
        } else {
            self.args.options = Some(vec![x]);
        }
        self
    }

    pub fn add_index(mut self, x: usize) -> Self {
        let x = Opt::Index(x);
        if let Some(o) = &mut self.args.options {
            o.push(x);
        } else {
            self.args.options = Some(vec![x]);
        }
        self
    }

    pub fn add_label(mut self, x: String) -> Self {
        let x = Opt::Label(x);
        if let Some(o) = &mut self.args.options {
            o.push(x);
        } else {
            self.args.options = Some(vec![x]);
        }
        self
    }

    optional_setter!(
        no_wait_after, bool;
        timeout, f64);

    pub fn clear_elements(mut self) -> Self {
        self.args.elements = None;
        self
    }

    pub fn clear_options(mut self) -> Self {
        self.args.options = None;
        self
    }
}

pub struct SetInputFilesBuilder {
    inner: Weak<Impl>,
    args: SetInputFilesArgs
}

impl SetInputFilesBuilder {
    pub(crate) fn new(inner: Weak<Impl>, file: File) -> Self {
        let args = SetInputFilesArgs {
            files: vec![file],
            ..SetInputFilesArgs::default()
        };
        Self { inner, args }
    }

    pub async fn set_input_files(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.set_input_files(args).await
    }

    pub fn add_file(mut self, x: File) -> Self {
        self.args.files.push(x);
        self
    }

    optional_setter!(
        no_wait_after, bool;
        timeout, f64);

    pub fn clear_files(mut self) -> Self {
        self.args.files = vec![];
        self
    }
}
