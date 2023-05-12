use crate::{
    api::Frame,
    imp::{
        core::*,
        element_handle::{
            CheckArgs, ClickArgs, ElementHandle as Impl, FillArgs, HoverArgs, Opt, PressArgs,
            ScreenshotArgs, SelectOptionArgs, SetInputFilePathsArgs, SetInputFilesArgs, TapArgs,
            TypeArgs, WaitForSelectorArgs
        },
        prelude::*,
        utils::{
            ElementState, File, FloatRect, KeyboardModifier, MouseButton, Position, ScreenshotType,
            WaitForSelectorState
        }
    }
};
use std::{borrow::Borrow, fs};

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
#[derive(Debug)]
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

    /// The method finds an element matching the specified selector in the `ElementHandle`'s subtree.
    /// If no elements match the selector, returns `null`.
    pub async fn query_selector(&self, selector: &str) -> ArcResult<Option<ElementHandle>> {
        Ok(upgrade(&self.inner)?
            .query_selector(selector)
            .await?
            .map(ElementHandle::new))
    }

    /// The method finds all elements matching the specified selector in the `ElementHandle`s subtree.
    /// If no elements match the selector, returns empty array.
    pub async fn query_selector_all(&self, selector: &str) -> ArcResult<Vec<ElementHandle>> {
        let es = upgrade(&self.inner)?.query_selector_all(selector).await?;
        Ok(es.into_iter().map(ElementHandle::new).collect())
    }

    /// Returns the `element.innerText`.
    pub async fn inner_text(&self) -> ArcResult<String> { upgrade(&self.inner)?.inner_text().await }

    /// Returns the `element.innerHTML`.
    pub async fn inner_html(&self) -> ArcResult<String> { upgrade(&self.inner)?.inner_html().await }

    is_checked! {is_checked}
    is_checked! {is_disabled}
    is_checked! {is_editable}
    is_checked! {is_enabled}
    is_checked! {is_hidden}
    is_checked! {is_visible}

    /// Returns the frame containing the given element.
    pub async fn owner_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.owner_frame().await?.map(Frame::new))
    }

    /// Returns the content frame for element handles referencing iframe nodes, or `null` otherwise
    pub async fn content_frame(&self) -> ArcResult<Option<Frame>> {
        Ok(upgrade(&self.inner)?.content_frame().await?.map(Frame::new))
    }

    /// Returns element attribute value.
    pub async fn get_attribute(&self, name: &str) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.get_attribute(name).await
    }

    /// Returns the `node.textContent`.
    pub async fn text_content(&self) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.text_content().await
    }

    /// This method hovers over the element by performing the following steps:
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to hover over the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    ///
    /// If the element is detached from the DOM at any moment during the action, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn hover_builder(&self) -> HoverBuilder { HoverBuilder::new(self.inner.clone()) }

    pub fn click_builder(&self) -> ClickBuilder { ClickBuilder::new(self.inner.clone()) }

    /// This method double clicks an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to double click in the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set. Note that if the
    ///   first click of the `dblclick()` triggers a navigation event, this method will throw.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    ///
    /// > NOTE: `frame.dblclick()` dispatches two `click` events and a single `dblclick` event.
    pub fn dblclick_builder(&self) -> DblClickBuilder { DblClickBuilder::new(self.inner.clone()) }

    /// This method checks the element by performing the following steps:
    /// 1. Ensure that element is a checkbox or a radio input. If not, this method throws. If the element is already checked,
    ///   this method returns immediately.
    /// 1. Wait for actionability checks on the element, unless `force` option is set.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to click in the center of the element.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    /// 1. Ensure that the element is now checked. If not, this method throws.
    ///
    /// If the element is detached from the DOM at any moment during the action, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn check_builder(&self) -> CheckBuilder { CheckBuilder::new(self.inner.clone()) }

    /// This method checks the element by performing the following steps:
    /// 1. Ensure that element is a checkbox or a radio input. If not, this method throws. If the element is already
    ///   unchecked, this method returns immediately.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to click in the center of the element.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    /// 1. Ensure that the element is now unchecked. If not, this method throws.
    ///
    /// If the element is detached from the DOM at any moment during the action, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn uncheck_builder(&self) -> UncheckBuilder { UncheckBuilder::new(self.inner.clone()) }

    /// This method taps the element by performing the following steps:
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the element, unless `force` option is set.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.touchscreen`] to tap the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    ///
    /// If the element is detached from the DOM at any moment during the action, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    ///
    /// > NOTE: `elementHandle.tap()` requires that the `hasTouch` option of the browser context be set to true.
    pub fn tap_builder(&self) -> TapBuilder { TapBuilder::new(self.inner.clone()) }

    /// This method waits for [actionability](https://playwright.dev/docs/actionability/) checks, focuses the element, fills it and triggers an `input`
    /// event after filling. Note that you can pass an empty string to clear the input field.
    ///
    /// If the target element is not an `<input>`, `<textarea>` or `[contenteditable]` element, this method throws an error.
    /// However, if the element is inside the `<label>` element that has an associated
    /// [control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), the control will be filled
    /// instead.
    ///
    /// To send fine-grained keyboard events, use [ElementHandle::type_builder](ElementHandle::type_builder)
    pub fn fill_builder<'a>(&self, value: &'a str) -> FillBuilder<'a> {
        FillBuilder::new(self.inner.clone(), value)
    }

    /// Calls [focus](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus) on the element.
    pub async fn focus(&self) -> ArcResult<()> { upgrade(&self.inner)?.focus().await }

    /// Focuses the element, and then sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text.
    ///
    /// To press a special key, like `Control` or `ArrowDown`, use [`method: ElementHandle.press`].
    ///
    /// ```js
    /// await elementHandle.type('Hello'); // Types instantly
    /// await elementHandle.type('World', {delay: 100}); // Types slower, like a user
    /// ```
    ///
    /// An example of typing into a text field and then submitting the form:
    ///
    /// ```js
    /// const elementHandle = await page.$('input');
    /// await elementHandle.type('some text');
    /// await elementHandle.press('Enter');
    /// ```
    pub fn type_builder<'a>(&self, text: &'a str) -> TypeBuilder<'a> {
        TypeBuilder::new(self.inner.clone(), text)
    }

    /// Focuses the element, and then uses [`method: Keyboard.down`] and [`method: Keyboard.up`].
    ///
    /// `key` can specify the intended [keyboardEvent.key](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    /// value or a single character to generate the text for. A superset of the `key` values can be found
    /// [here](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values). Examples of the keys are:
    ///
    /// `F1` - `F12`, `Digit0`- `Digit9`, `KeyA`- `KeyZ`, `Backquote`, `Minus`, `Equal`, `Backslash`, `Backspace`, `Tab`,
    /// `Delete`, `Escape`, `ArrowDown`, `End`, `Enter`, `Home`, `Insert`, `PageDown`, `PageUp`, `ArrowRight`, `ArrowUp`, etc.
    ///
    /// Following modification shortcuts are also supported: `Shift`, `Control`, `Alt`, `Meta`, `ShiftLeft`.
    ///
    /// Holding down `Shift` will type the text that corresponds to the `key` in the upper case.
    ///
    /// If `key` is a single character, it is case-sensitive, so the values `a` and `A` will generate different respective
    /// texts.
    ///
    /// Shortcuts such as `key: "Control+o"` or `key: "Control+Shift+T"` are supported as well. When specified with the
    /// modifier, modifier is pressed and being held while the subsequent key is being pressed.
    pub fn press_builder<'a>(&self, key: &'a str) -> PressBuilder<'a> {
        PressBuilder::new(self.inner.clone(), key)
    }

    /// This method waits for actionability checks, then tries to scroll element into view, unless it is
    /// completely visible as defined by
    /// [IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API)'s `ratio`.
    ///
    /// Throws when `elementHandle` does not point to an element
    /// [connected](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected) to a Document or a ShadowRoot.
    pub async fn scroll_into_view_if_needed(&self, timeout: Option<f64>) -> ArcResult<()> {
        upgrade(&self.inner)?
            .scroll_into_view_if_needed(timeout)
            .await
    }

    /// This method waits for actionability checks, then focuses the element and selects all its text
    /// content.
    pub async fn select_text(&self, timeout: Option<f64>) -> ArcResult<()> {
        upgrade(&self.inner)?.select_text(timeout).await
    }

    /// This method returns the bounding box of the element, or `null` if the element is not visible. The bounding box is
    /// calculated relative to the main frame viewport - which is usually the same as the browser window.
    ///
    /// Scrolling affects the returned bonding box, similarly to
    /// [Element.getBoundingClientRect](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect). That
    /// means `x` and/or `y` may be negative.
    ///
    /// Elements from child frames return the bounding box relative to the main frame, unlike the
    /// [Element.getBoundingClientRect](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect).
    ///
    /// Assuming the page is static, it is safe to use bounding box coordinates to perform input. For example, the following
    /// snippet should click the center of the element.
    ///
    /// ```js
    /// const box = await elementHandle.boundingBox();
    /// await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    /// ```
    pub async fn bounding_box(&self) -> ArcResult<Option<FloatRect>> {
        upgrade(&self.inner)?.bounding_box().await
    }

    /// Returns the buffer with the captured screenshot.
    ///
    /// This method waits for the actionability checks, then scrolls element into view before taking a
    /// screenshot. If the element is detached from DOM, the method throws an error.
    pub async fn screenshot_builder(&self) -> ScreenshotBuilder<'_> {
        ScreenshotBuilder::new(self.inner.clone())
    }

    /// Returns when the element satisfies the `state`.
    pub async fn wait_for_element_state(
        &self,
        state: ElementState,
        timeout: Option<f64>
    ) -> ArcResult<()> {
        upgrade(&self.inner)?
            .wait_for_element_state(state, timeout)
            .await
    }

    /// Returns when element specified by selector satisfies `state` option. Returns `null` if waiting for `hidden` or
    /// `detached`.
    ///
    /// Wait for the `selector` to satisfy `state` option (either appear/disappear from dom, or become visible/hidden). If at
    /// the moment of calling the method `selector` already satisfies the condition, the method will return immediately. If the
    /// selector doesn't satisfy the condition for the `timeout` milliseconds, the function will throw.
    ///
    /// This method works across navigations:
    ///
    /// ```js
    /// const { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.
    ///
    /// (async () => {
    ///  const browser = await chromium.launch();
    ///  const page = await browser.newPage();
    ///  for (let currentURL of ['https://google.com', 'https://bbc.com']) {
    ///    await page.goto(currentURL);
    ///    const element = await page.mainFrame().waitForSelector('img');
    ///    console.log('Loaded image: ' + await element.getAttribute('src'));
    ///  }
    ///  await browser.close();
    /// })();
    /// ```
    pub fn wait_for_selector_builder<'a>(&self, selector: &'a str) -> WaitForSelectorBuilder<'a> {
        WaitForSelectorBuilder::new(self.inner.clone(), selector)
    }

    /// The snippet below dispatches the `click` event on the element. Regardless of the visibility state of the element,
    /// `click` is dispatched. This is equivalent to calling
    /// [element.click()](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click).
    ///
    /// ```js
    /// await elementHandle.dispatchEvent('click');
    /// ```
    ///
    /// Under the hood, it creates an instance of an event based on the given `type`, initializes it with `eventInit` properties
    /// and dispatches it on the element. Events are `composed`, `cancelable` and bubble by default.
    ///
    /// Since `eventInit` is event-specific, please refer to the events documentation for the lists of initial properties:
    /// - [DragEvent](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)
    /// - [FocusEvent](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)
    /// - [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)
    /// - [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)
    /// - [PointerEvent](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)
    /// - [TouchEvent](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)
    /// - [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)
    ///
    /// You can also specify `JSHandle` as the property value if you want live objects to be passed into the event:
    ///
    /// ```js
    ///// Note you can only create DataTransfer in Chromium and Firefox
    /// const dataTransfer = await page.evaluateHandle(() => new DataTransfer());
    /// await elementHandle.dispatchEvent('dragstart', { dataTransfer });
    /// ```
    pub async fn dispatch_event<T>(&self, r#type: &str, event_init: Option<T>) -> ArcResult<()>
    where
        T: Serialize
    {
        upgrade(&self.inner)?
            .dispatch_event(r#type, event_init)
            .await
    }

    /// This method waits for [actionability](https://playwright.dev/docs/actionability/) checks, waits until all specified options are present in the
    /// `<select>` element and selects these options.
    ///
    /// If the target element is not a `<select>` element, this method throws an error. However, if the element is inside the
    /// `<label>` element that has an associated
    /// [control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), the control will be used instead.
    ///
    /// Returns the array of option values that have been successfully selected.
    ///
    /// Triggers a `change` and `input` event once all the provided options have been selected.
    ///
    /// ```js
    ///// single selection matching the value
    /// handle.selectOption('blue');
    ///// single selection matching the label
    /// handle.selectOption({ label: 'Blue' });
    ///// multiple selection
    /// handle.selectOption(['red', 'green', 'blue']);
    /// ```
    pub fn select_option_builder(&self) -> SelectOptionBuilder {
        SelectOptionBuilder::new(self.inner.clone())
    }

    /// This method expects `elementHandle` to point to an
    /// [input element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    ///
    /// Sets the value of the file input to these file paths or files. If some of the `filePaths` are relative paths, then they
    /// are resolved relative to the the current working directory. For empty array, clears the selected files.
    pub fn set_input_files_builder(&self, file: File) -> SetInputFilesBuilder {
        SetInputFilesBuilder::new(self.inner.clone(), file)
    }

    pub fn set_input_file_paths_builder(&self, file: &str) -> SetInputFilePathsBuilder {
        SetInputFilePathsBuilder::new(self.inner.clone(), file)
    }
    // eval_on_selector
    // eval_on_selector_all
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

    setter! {
        /// Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`.
        force: Option<bool>,
        /// Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current
        /// modifiers back. If not specified, currently pressed modifiers are used.
        modifiers: Option<Vec<KeyboardModifier>>,
        /// A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the
        /// element.
        position: Option<Position>,
        timeout: Option<f64>,
        /// When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to
        /// `false`. Useful to wait until the element is ready for the action without performing it.
        trial: Option<bool>
    }
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

            setter! {
                /// Defaults to `left`.
                button: Option<MouseButton>,
                /// Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0.
                delay: Option<f64>,
                /// Whether to bypass the [actionability](https://playwright.dev/docs/actionability/) checks. Defaults to `false`.
                force: Option<bool>,
                /// Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current
                /// modifiers back. If not specified, currently pressed modifiers are used.
                modifiers: Option<Vec<KeyboardModifier>>,
                /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
                /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
                /// inaccessible pages. Defaults to `false`.
                no_wait_after: Option<bool>,
                /// A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the
                /// element.
                position: Option<Position>,
                timeout: Option<f64>,
                /// When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to
                /// `false`. Useful to wait until the element is ready for the action without performing it.
                trial: Option<bool>
            }
        }
    };
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

            setter! {
                /// A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the element.
                position: Option<Position>,
                /// Whether to bypass the actionability checks. Defaults to `false`.
                force: Option<bool>,
                /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
                /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
                /// inaccessible pages. Defaults to `false`.
                no_wait_after: Option<bool>,
                timeout: Option<f64>,
                /// When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to
                /// `false`. Useful to wait until the element is ready for the action without performing it.
                trial: Option<bool>
            }
        }
    };
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

    setter! {
        /// Whether to bypass the actionability checks. Defaults to `false`.
        force: Option<bool>,
        /// Modifier keys to press. Ensures that only these modifiers are pressed during the operation, and then restores current
        /// modifiers back. If not specified, currently pressed modifiers are used.
        modifiers: Option<Vec<KeyboardModifier>>,
        /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
        /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
        /// inaccessible pages. Defaults to `false`.
        no_wait_after: Option<bool>,
        /// A point to use relative to the top-left corner of element padding box. If not specified, uses some visible point of the
        /// element.
        position: Option<Position>,
        timeout: Option<f64>,
        /// When set, this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Defaults to
        /// `false`. Useful to wait until the element is ready for the action without performing it.
        trial: Option<bool>
    }
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

    setter! {
        /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
        /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
        /// inaccessible pages. Defaults to `false`.
        no_wait_after: Option<bool>,
        timeout: Option<f64>
    }
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

            setter! {
                /// Time to wait between `keydown` and `keyup` in milliseconds. Defaults to 0.
                delay: Option<f64>,
                /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
                /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
                /// inaccessible pages. Defaults to `false`.
                no_wait_after: Option<bool>,
                timeout: Option<f64>
            }
        }
    };
}

type_builder!(TypeBuilder, TypeArgs, text, r#type);
type_builder!(PressBuilder, PressArgs, key, press);

pub struct ScreenshotBuilder<'a> {
    inner: Weak<Impl>,
    args: ScreenshotArgs<'a>
}

impl<'a> ScreenshotBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        let args = ScreenshotArgs::default();
        Self { inner, args }
    }

    pub async fn screenshot(self) -> ArcResult<Vec<u8>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.screenshot(args).await
    }

    /// Specify screenshot type, defaults to `png`.
    pub fn r#type(mut self, x: ScreenshotType) -> Self {
        self.args.r#type = Some(x);
        self
    }

    setter! {
        /// Hides default white background and allows capturing screenshots with transparency. Not applicable to `jpeg` images.
        /// Defaults to `false`.
        omit_background: Option<bool>,
        /// The file path to save the image to. The screenshot type will be inferred from file extension. If `path` is a relative
        /// path, then it is resolved relative to the current working directory. If no path is provided, the image won't be saved to
        /// the disk.
        path: Option<&'a Path>,
        quality: Option<i64>,
        timeout: Option<f64>
    }

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

    setter! {
        state: Option<WaitForSelectorState>,
        timeout: Option<f64>
    }
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

    setter! {
        /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
        /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
        /// inaccessible pages. Defaults to `false`.
        no_wait_after: Option<bool>,
        timeout: Option<f64>
    }

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

    setter! {
        /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
        /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
        /// inaccessible pages. Defaults to `false`.
        no_wait_after: Option<bool>,
        timeout: Option<f64>
    }

    pub fn clear_files(mut self) -> Self {
        self.args.files = vec![];
        self
    }
}

pub struct SetInputFilePathsBuilder {
    inner: Weak<Impl>,
    args: SetInputFilePathsArgs
}

impl SetInputFilePathsBuilder {
    pub(crate) fn new(inner: Weak<Impl>, filepath: &str) -> Self {
        let f = fs::canonicalize(filepath).unwrap();
        let args = SetInputFilePathsArgs {
            local_paths: Some(vec![f]),
            ..SetInputFilePathsArgs::default()
        };
        Self { inner, args }
    }

    pub async fn set_input_file_paths(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.set_input_file_paths(args).await
    }

    pub fn add_file(mut self, x: &str) -> Self {
        let mut local_paths = self.args.local_paths.as_mut().unwrap();
        local_paths.push(fs::canonicalize(x).unwrap());
        self
    }

    setter! {
        no_wait_after: Option<bool>,
        timeout: Option<f64>
    }

    pub fn clear_files(mut self) -> Self {
        self.args.local_paths = Some(vec![]);
        self
    }
}

mod ser {
    use super::*;
    use serde::{ser, ser::SerializeStruct};

    impl Serialize for ElementHandle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer
        {
            let mut s = serializer.serialize_struct("fff9ae7f-9070-480f-9a8a-3d4b66923f7d", 1)?;
            let guid = &self.guid().map_err(<S::Error as ser::Error>::custom)?;
            s.serialize_field("guid", &guid)?;
            s.end()
        }
    }
}
