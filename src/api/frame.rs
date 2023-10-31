pub use crate::imp::frame::{FrameNavigatedEvent, FrameState, Polling};
use crate::{
    api::{ElementHandle, JsHandle, Page, Response},
    imp::{
        core::*,
        frame::{
            AddScriptTagArgs, CheckArgs, ClickArgs, Evt, FillArgs, Frame as Impl, GotoArgs,
            HoverArgs, Opt, PressArgs, SelectOptionArgs, SetContentArgs, SetInputFilesArgs,
            TapArgs, TypeArgs, WaitForFunctionArgs, WaitForSelectorArgs
        },
        prelude::*,
        utils::{DocumentLoadState, File, KeyboardModifier, MouseButton, Position}
    },
    protocol::generated::LifecycleEvent
};

/// At every point of time, page exposes its current frame tree via the [`method: Page.mainFrame`] and
/// [`method: Frame.childFrames`] methods.
///
/// `Frame` object's lifecycle is controlled by three events, dispatched on the page object:
/// - [`event: Page.frameAttached`] - fired when the frame gets attached to the page. A Frame can be attached to the page
///  only once.
/// - [`event: Page.frameNavigated`] - fired when the frame commits navigation to a different URL.
/// - [`event: Page.frameDetached`] - fired when the frame gets detached from the page.  A Frame can be detached from the
///  page only once.
///
/// An example of dumping frame tree:
///
/// ```js
/// const { firefox } = require('playwright');  // Or 'chromium' or 'webkit'.
///
/// (async () => {
///  const browser = await firefox.launch();
///  const page = await browser.newPage();
///  await page.goto('https://www.google.com/chrome/browser/canary.html');
///  dumpFrameTree(page.mainFrame(), '');
///  await browser.close();
///
///  function dumpFrameTree(frame, indent) {
///    console.log(indent + frame.url());
///    for (const child of frame.childFrames()) {
///      dumpFrameTree(child, indent + '  ');
///    }
///  }
/// })();
/// ```
#[derive(Clone)]
pub struct Frame {
    inner: Weak<Impl>
}

impl PartialEq for Frame {
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
        pub async fn $f(&self, selector: &str, timeout: Option<f64>) -> ArcResult<bool> {
            upgrade(&self.inner)?.$f(selector, timeout).await
        }
    };
}

impl Frame {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url()) }

    /// Returns frame's name attribute as specified in the tag.
    ///
    /// If the name is empty, returns the id attribute instead.
    ///
    /// > NOTE: This value is calculated once when the frame is created, and will not update if the attribute is changed later.
    pub fn name(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.name()) }

    pub fn page(&self) -> Result<Option<Page>, Error> {
        Ok(upgrade(&self.inner)?.page().map(Page::new))
    }

    /// Parent frame, if any. Detached frames and main frames return `null`.
    pub fn parent_frame(&self) -> Result<Option<Frame>, Error> {
        Ok(upgrade(&self.inner)?.parent_frame().map(Frame::new))
    }

    pub fn child_frames(&self) -> Result<Vec<Frame>, Error> {
        Ok(upgrade(&self.inner)?
            .child_frames()
            .into_iter()
            .map(Frame::new)
            .collect())
    }

    /// Returns the main resource response. In case of multiple redirects, the navigation will resolve with the response of the
    /// last redirect.
    ///
    /// `frame.goto` will throw an error if:
    /// - there's an SSL error (e.g. in case of self-signed certificates).
    /// - target URL is invalid.
    /// - the `timeout` is exceeded during navigation.
    /// - the remote server does not respond or is unreachable.
    /// - the main resource failed to load.
    ///
    /// `frame.goto` will not throw an error when any valid HTTP status code is returned by the remote server, including 404
    /// "Not Found" and 500 "Internal Server Error".  The status code for such responses can be retrieved by calling
    /// [`method: Response.status`].
    ///
    /// > NOTE: `frame.goto` either throws an error or returns a main resource response. The only exceptions are navigation to
    /// `about:blank` or navigation to the same URL with a different hash, which would succeed and return `null`.
    /// > NOTE: Headless mode doesn't support navigation to a PDF document. See the
    /// [upstream issue](https://bugs.chromium.org/p/chromium/issues/detail?id=761295).
    pub fn goto_builder<'a>(&self, url: &'a str) -> GotoBuilder<'a, '_> {
        GotoBuilder::new(self.inner.clone(), url)
    }

    /// This method clicks an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to click in the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn click_builder<'a>(&self, selector: &'a str) -> ClickBuilder<'a> {
        ClickBuilder::new(self.inner.clone(), selector)
    }

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
    pub fn dblclick_builder<'a>(&self, selector: &'a str) -> DblClickBuilder<'a> {
        DblClickBuilder::new(self.inner.clone(), selector)
    }

    /// This method taps an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.touchscreen`] to tap the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    ///
    /// > NOTE: `frame.tap()` requires that the `hasTouch` option of the browser context be set to true.
    pub fn tap_builder<'a>(&self, selector: &'a str) -> TapBuilder<'a> {
        TapBuilder::new(self.inner.clone(), selector)
    }
    /// This method waits for an element matching `selector`, waits for [actionability](https://playwright.dev/docs/actionability/) checks, focuses the
    /// element, fills it and triggers an `input` event after filling. Note that you can pass an empty string to clear the input
    /// field.
    ///
    /// If the target element is not an `<input>`, `<textarea>` or `[contenteditable]` element, this method throws an error.
    /// However, if the element is inside the `<label>` element that has an associated
    /// [control](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control), the control will be filled
    /// instead.
    ///
    /// To send fine-grained keyboard events, use [Frame::type_builder](Frame::type_builder).
    pub fn fill_builder<'a, 'b>(&self, selector: &'a str, value: &'b str) -> FillBuilder<'a, 'b> {
        FillBuilder::new(self.inner.clone(), selector, value)
    }

    /// This method fetches an element with `selector` and focuses it. If there's no element matching `selector`, the method
    /// waits until a matching element appears in the DOM.
    pub async fn focus(&self, selector: &str, timeout: Option<f64>) -> ArcResult<()> {
        upgrade(&self.inner)?.focus(selector, timeout).await
    }

    /// Returns `element.textContent`.
    pub async fn text_content(
        &self,
        selector: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?.text_content(selector, timeout).await
    }

    /// Returns `element.innerText`.
    pub async fn inner_text(&self, selector: &str, timeout: Option<f64>) -> ArcResult<String> {
        upgrade(&self.inner)?.inner_text(selector, timeout).await
    }

    /// Returns `element.innerHTML`.
    pub async fn inner_html(&self, selector: &str, timeout: Option<f64>) -> ArcResult<String> {
        upgrade(&self.inner)?.inner_html(selector, timeout).await
    }

    /// Returns element attribute value.
    pub async fn get_attribute(
        &self,
        selector: &str,
        name: &str,
        timeout: Option<f64>
    ) -> ArcResult<Option<String>> {
        upgrade(&self.inner)?
            .get_attribute(selector, name, timeout)
            .await
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

    /// Returns the `frame` or `iframe` element handle which corresponds to this frame.
    ///
    /// This is an inverse of [`method: ElementHandle.contentFrame`]. Note that returned handle actually belongs to the parent
    /// frame.
    ///
    /// This method throws an error if the frame has been detached before `frameElement()` returns.
    ///
    /// ```js
    /// const frameElement = await frame.frameElement();
    /// const contentFrame = await frameElement.contentFrame();
    /// console.log(frame === contentFrame);  // -> true
    /// ```
    pub async fn frame_element(&self) -> ArcResult<ElementHandle> {
        Ok(ElementHandle::new(
            upgrade(&self.inner)?.frame_element().await?
        ))
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

    pub async fn title(&self) -> ArcResult<String> { upgrade(&self.inner)?.title().await }

    /// Sends a `keydown`, `keypress`/`input`, and `keyup` event for each character in the text. `frame.type` can be used to
    /// send fine-grained keyboard events. To fill values in form fields, use [`method: Frame.fill`].
    ///
    /// To press a special key, like `Control` or `ArrowDown`, use [`method: Keyboard.press`].
    ///
    /// ```js
    /// await frame.type('#mytextarea', 'Hello'); // Types instantly
    /// await frame.type('#mytextarea', 'World', {delay: 100}); // Types slower, like a user
    /// ```
    pub fn type_builder<'a, 'b>(&self, selector: &'a str, text: &'b str) -> TypeBuilder<'a, 'b> {
        TypeBuilder::new(self.inner.clone(), selector, text)
    }

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
    pub fn press_builder<'a, 'b>(&self, selector: &'a str, key: &'b str) -> PressBuilder<'a, 'b> {
        PressBuilder::new(self.inner.clone(), selector, key)
    }

    /// This method hovers over an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to hover over the center of the element, or the specified `position`.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn hover_builder<'a>(&self, selector: &'a str) -> HoverBuilder<'a> {
        HoverBuilder::new(self.inner.clone(), selector)
    }

    is_checked! {is_checked}
    is_checked! {is_disabled}
    is_checked! {is_editable}
    is_checked! {is_enabled}
    is_checked! {is_hidden}
    is_checked! {is_visible}

    /// Gets the full HTML contents of the frame, including the doctype.
    pub async fn content<'a>(&self) -> ArcResult<String> { upgrade(&self.inner)?.content().await }

    pub fn set_content_builder<'a>(&self, html: &'a str) -> SetContentBuilder<'a> {
        SetContentBuilder::new(self.inner.clone(), html)
    }

    /// This method checks an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already
    ///   checked, this method returns immediately.
    /// 1. Wait for actionability checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to click in the center of the element.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    /// 1. Ensure that the element is now checked. If not, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn check_builder<'a>(&self, selector: &'a str) -> CheckBuilder<'a> {
        CheckBuilder::new(self.inner.clone(), selector)
    }

    /// This method checks an element matching `selector` by performing the following steps:
    /// 1. Find an element matching `selector`. If there is none, wait until a matching element is attached to the DOM.
    /// 1. Ensure that matched element is a checkbox or a radio input. If not, this method throws. If the element is already
    ///   unchecked, this method returns immediately.
    /// 1. Wait for [actionability](https://playwright.dev/docs/actionability/) checks on the matched element, unless `force` option is set. If the
    ///   element is detached during the checks, the whole action is retried.
    /// 1. Scroll the element into view if needed.
    /// 1. Use [`property: Page.mouse`] to click in the center of the element.
    /// 1. Wait for initiated navigations to either succeed or fail, unless `noWaitAfter` option is set.
    /// 1. Ensure that the element is now unchecked. If not, this method throws.
    ///
    /// When all steps combined have not finished during the specified `timeout`, this method throws a `TimeoutError`. Passing
    /// zero timeout disables this.
    pub fn uncheck_builder<'a>(&self, selector: &'a str) -> UncheckBuilder<'a> {
        UncheckBuilder::new(self.inner.clone(), selector)
    }

    // = |timeout| async { sleep(timeout).await }
    pub async fn wait_for_timeout(&self, timeout: f64) {
        sleep(std::time::Duration::from_millis(timeout as u64)).await
    }

    /// Returns the added tag when the stylesheet's onload fires or when the CSS content was injected into frame.
    ///
    /// Adds a `<link rel="stylesheet">` tag into the page with the desired url or a `<style type="text/css">` tag with the
    /// content.
    pub async fn add_style_tag(
        &self,
        content: &str,
        url: Option<&str>
    ) -> ArcResult<ElementHandle> {
        upgrade(&self.inner)?
            .add_style_tag(content, url)
            .await
            .map(ElementHandle::new)
    }

    /// Returns the added tag when the script's onload fires or when the script content was injected into frame.
    ///
    /// Adds a `<script>` tag into the page with the desired url or content.
    pub fn add_script_tag_builder<'a>(&self, content: &'a str) -> AddScriptTagBuilder<'a, '_, '_> {
        AddScriptTagBuilder::new(self.inner.clone(), content)
    }

    pub async fn evaluate_element_handle<T>(
        &self,
        expression: &str,
        args: Option<T>
    ) -> ArcResult<ElementHandle>
    where
        T: Serialize
    {
        upgrade(&self.inner)?
            .evaluate_element_handle(expression, args)
            .await
            .map(ElementHandle::new)
    }

    /// Returns the return value of `expression` as a `JSHandle`.
    ///
    /// The only difference between [`method: Frame.evaluate`] and [`method: Frame.evaluateHandle`] is that
    /// [`method: Frame.evaluateHandle`] returns `JSHandle`.
    ///
    /// If the function, passed to the [`method: Frame.evaluateHandle`], returns a Promise, then
    /// [`method: Frame.evaluateHandle`] would wait for the promise to resolve and return its value.
    ///
    /// ```js
    /// const aWindowHandle = await frame.evaluateHandle(() => Promise.resolve(window));
    /// aWindowHandle; // Handle for the window object.
    /// ```
    ///
    /// A string can also be passed in instead of a function.
    /// ```js
    /// const aHandle = await frame.evaluateHandle('document'); // Handle for the 'document'.
    /// ```
    ///
    /// `JSHandle` instances can be passed as an argument to the [`method: Frame.evaluateHandle`]:
    /// ```js
    /// const aHandle = await frame.evaluateHandle(() => document.body);
    /// const resultHandle = await frame.evaluateHandle(([body, suffix]) => body.innerHTML + suffix, [aHandle, 'hello']);
    /// console.log(await resultHandle.jsonValue());
    /// await resultHandle.dispose();
    /// ```
    pub async fn evaluate_js_handle<T>(
        &self,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<JsHandle>
    where
        T: Serialize
    {
        upgrade(&self.inner)?
            .evaluate_js_handle(expression, arg)
            .await
            .map(JsHandle::new)
    }

    pub async fn eval<U>(&self, expression: &str) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.eval(expression).await
    }

    /// Returns the return value of `expression`.
    ///
    /// If the function passed to the [`method: Frame.evaluate`] returns a Promise, then [`method: Frame.evaluate`] would wait
    /// for the promise to resolve and return its value.
    ///
    /// If the function passed to the [`method: Frame.evaluate`] returns a non-Serializable value, then
    /// [`method: Frame.evaluate`] returns `undefined`. Playwright also supports transferring some additional values that are
    /// not serializable by `JSON`: `-0`, `NaN`, `Infinity`, `-Infinity`.
    ///
    /// ```js
    /// const result = await frame.evaluate(([x, y]) => {
    ///  return Promise.resolve(x * y);
    /// }, [7, 8]);
    /// console.log(result); // prints "56"
    /// ```
    ///
    /// `ElementHandle` instances can be passed as an argument to the [`method: Frame.evaluate`]:
    ///
    /// ```js
    /// const bodyHandle = await frame.$('body');
    /// const html = await frame.evaluate(([body, suffix]) => body.innerHTML + suffix, [bodyHandle, 'hello']);
    /// await bodyHandle.dispose();
    /// ```
    pub async fn evaluate<T, U>(&self, expression: &str, arg: T) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.evaluate(expression, Some(arg)).await
    }

    /// Returns the return value of `expression`.
    ///
    /// The method finds an element matching the specified selector within the frame and passes it as a first argument to
    /// `expression`. If no elements match the selector, the
    /// method throws an error.
    ///
    /// If `expression` returns a Promise, then [`method: Frame.evalOnSelector`] would wait for the promise to resolve and
    /// return its value.
    ///
    /// Examples:
    ///
    /// ```js
    /// const searchValue = await frame.$eval('#search', el => el.value);
    /// const preloadHref = await frame.$eval('link[rel=preload]', el => el.href);
    /// const html = await frame.$eval('.main-container', (e, suffix) => e.outerHTML + suffix, 'hello');
    /// ```
    pub async fn evaluate_on_selector<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        upgrade(&self.inner)?
            .evaluate_on_selector(selector, expression, arg)
            .await
    }

    /// Returns the return value of `expression`.
    ///
    /// The method finds all elements matching the specified selector within the frame and passes an array of matched elements
    /// as a first argument to `expression`.
    ///
    /// If `expression` returns a Promise, then [`method: Frame.evalOnSelectorAll`] would wait for the promise to resolve and
    /// return its value.
    ///
    /// Examples:
    ///
    /// ```js
    /// const divsCounts = await frame.$$eval('div', (divs, min) => divs.length >= min, 10);
    /// ```
    pub async fn evaluate_on_selector_all<T, U>(
        &self,
        selector: &str,
        expression: &str,
        arg: Option<T>
    ) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        upgrade(&self.inner)?
            .evaluate_on_selector_all(selector, expression, arg)
            .await
    }

    /// The snippet below dispatches the `click` event on the element. Regardless of the visibility state of the element,
    /// `click` is dispatched. This is equivalent to calling
    /// [element.click()](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click).
    ///
    /// ```js
    /// await frame.dispatchEvent('button#submit', 'click');
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
    /// const dataTransfer = await frame.evaluateHandle(() => new DataTransfer());
    /// await frame.dispatchEvent('#source', 'dragstart', { dataTransfer });
    /// ```
    pub async fn dispatch_event<T>(
        &self,
        selector: &str,
        r#type: &str,
        event_init: Option<T>
    ) -> ArcResult<()>
    where
        T: Serialize
    {
        // timeout not supported
        upgrade(&self.inner)?
            .dispatch_event(selector, r#type, event_init)
            .await
    }

    /// This method waits for an element matching `selector`, waits for [actionability](https://playwright.dev/docs/actionability/) checks, waits until
    /// all specified options are present in the `<select>` element and selects these options.
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
    /// frame.selectOption('select#colors', 'blue');
    ///// single selection matching both the value and the label
    /// frame.selectOption('select#colors', { label: 'Blue' });
    ///// multiple selection
    /// frame.selectOption('select#colors', 'red', 'green', 'blue');
    /// ```
    pub fn select_option_builder<'a>(&self, selector: &'a str) -> SelectOptionBuilder<'a> {
        SelectOptionBuilder::new(self.inner.clone(), selector)
    }

    /// This method expects `selector` to point to an
    /// [input element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    ///
    /// Sets the value of the file input to these file paths or files. If some of the `filePaths` are relative paths, then they
    /// are resolved relative to the the current working directory. For empty array, clears the selected files.
    pub fn set_input_files_builder<'a>(
        &self,
        selector: &'a str,
        file: File
    ) -> SetInputFilesBuilder<'a> {
        SetInputFilesBuilder::new(self.inner.clone(), selector, file)
    }

    /// Returns when the `expression` returns a truthy value, returns that value.
    ///
    /// The [`method: Frame.waitForFunction`] can be used to observe viewport size change:
    ///
    /// ```js
    /// const { firefox } = require('playwright');  // Or 'chromium' or 'webkit'.
    ///
    /// (async () => {
    ///  const browser = await firefox.launch();
    ///  const page = await browser.newPage();
    ///  const watchDog = page.mainFrame().waitForFunction('window.innerWidth < 100');
    ///  page.setViewportSize({width: 50, height: 50});
    ///  await watchDog;
    ///  await browser.close();
    /// })();
    /// ```
    pub fn wait_for_function_builder<'a>(&self, expression: &'a str) -> WaitForFunctionBuilder<'a> {
        WaitForFunctionBuilder::new(self.inner.clone(), expression)
    }

    subscribe_event! {}

    // wait_for_url
}

#[derive(Debug)]
pub enum Event {
    LoadState(LifecycleEvent),
    Navigated(FrameNavigatedEvent)
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Self {
        match e {
            Evt::LoadState(x) => Self::LoadState(x),
            Evt::Navigated(x) => Self::Navigated(x)
        }
    }
}

pub struct GotoBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: GotoArgs<'a, 'b>
}

impl<'a, 'b> GotoBuilder<'a, 'b> {
    pub(crate) fn new(inner: Weak<Impl>, url: &'a str) -> Self {
        let args = GotoArgs::new(url);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<Option<Response>, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.goto(args).await?;
        Ok(r.map(Response::new))
    }

    setter! {
        /// Referer header value. If provided it will take preference over the referer header value set by
        /// [`method: Page.setExtraHTTPHeaders`].
        referer: Option<&'b str>,
        timeout: Option<f64>,
        wait_until: Option<DocumentLoadState>
    }
}

macro_rules! clicker {
    ($t: ident, $f: ident) => {
        pub struct $t<'a> {
            inner: Weak<Impl>,
            args: ClickArgs<'a>
        }

        impl<'a> $t<'a> {
            pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
                let args = ClickArgs::new(selector);
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
                /// defaults to 1. See [UIEvent.detail].
                click_count: Option<i32>,
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

pub struct WaitForSelectorBuilder<'a> {
    inner: Weak<Impl>,
    args: WaitForSelectorArgs<'a>
}

impl<'a> WaitForSelectorBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = WaitForSelectorArgs::new(selector);
        Self { inner, args }
    }

    pub async fn wait_for_selector(self) -> Result<Option<ElementHandle>, Arc<Error>> {
        let Self { inner, args } = self;
        let e = upgrade(&inner)?.wait_for_selector(args).await?;
        Ok(e.map(ElementHandle::new))
    }

    setter! {
        /// Defaults to `'visible'`.
        state: Option<FrameState>,
        timeout: Option<f64>
    }
}

macro_rules! type_builder {
    ($t: ident, $a: ident, $f: ident, $m: ident) => {
        pub struct $t<'a, 'b> {
            inner: Weak<Impl>,
            args: $a<'a, 'b>
        }

        impl<'a, 'b> $t<'a, 'b> {
            pub(crate) fn new(inner: Weak<Impl>, selector: &'a str, $f: &'b str) -> Self {
                let args = $a::new(selector, $f);
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

pub struct HoverBuilder<'a> {
    inner: Weak<Impl>,
    args: HoverArgs<'a>
}

impl<'a> HoverBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = HoverArgs::new(selector);
        Self { inner, args }
    }

    pub async fn goto(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.hover(args).await
    }

    setter! {
        /// Whether to bypass the actionability checks. Defaults to `false`.
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

pub struct SetContentBuilder<'a> {
    inner: Weak<Impl>,
    args: SetContentArgs<'a>
}

impl<'a> SetContentBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, html: &'a str) -> Self {
        let args = SetContentArgs::new(html);
        Self { inner, args }
    }

    pub async fn set_content(self) -> Result<(), Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.set_content(args).await
    }

    setter! {
        timeout: Option<f64>,
        wait_until: Option<DocumentLoadState>
    }
}

pub struct TapBuilder<'a> {
    inner: Weak<Impl>,
    args: TapArgs<'a>
}

impl<'a> TapBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = TapArgs::new(selector);
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

pub struct FillBuilder<'a, 'b> {
    inner: Weak<Impl>,
    args: FillArgs<'a, 'b>
}

impl<'a, 'b> FillBuilder<'a, 'b> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str, value: &'b str) -> Self {
        let args = FillArgs::new(selector, value);
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

macro_rules! check_builder {
    ($t: ident, $m: ident) => {
        pub struct $t<'a> {
            inner: Weak<Impl>,
            args: CheckArgs<'a>
        }

        impl<'a> $t<'a> {
            pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
                let args = CheckArgs::new(selector);
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

pub struct AddScriptTagBuilder<'a, 'b, 'c> {
    inner: Weak<Impl>,
    args: AddScriptTagArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> AddScriptTagBuilder<'a, 'b, 'c> {
    pub(crate) fn new(inner: Weak<Impl>, content: &'a str) -> Self {
        let args = AddScriptTagArgs::new(content);
        Self { inner, args }
    }

    pub async fn add_script_tag(self) -> Result<ElementHandle, Arc<Error>> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .add_script_tag(args)
            .await
            .map(ElementHandle::new)
    }

    /// Script type. Use 'module' in order to load a Javascript ES6 module. See
    /// [script](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script) for more details.
    pub fn r#type(mut self, x: &'c str) -> Self {
        self.args.r#type = Some(x);
        self
    }

    setter! {
        /// URL of a script to be added.
        url: Option<&'b str>
    }

    pub fn clear_type(mut self) -> Self {
        self.args.r#type = None;
        self
    }
}

pub struct SelectOptionBuilder<'a> {
    inner: Weak<Impl>,
    args: SelectOptionArgs<'a>,
    err: Option<Error>
}

impl<'a> SelectOptionBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str) -> Self {
        let args = SelectOptionArgs::new(selector);
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

pub struct SetInputFilesBuilder<'a> {
    inner: Weak<Impl>,
    args: SetInputFilesArgs<'a>
}

impl<'a> SetInputFilesBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, selector: &'a str, file: File) -> Self {
        let mut args = SetInputFilesArgs::new(selector);
        args.files = vec![file];
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

pub struct WaitForFunctionBuilder<'a> {
    inner: Weak<Impl>,
    args: WaitForFunctionArgs<'a>,
    err: Option<Error>
}

impl<'a> WaitForFunctionBuilder<'a> {
    pub(crate) fn new(inner: Weak<Impl>, expression: &'a str) -> Self {
        let args = WaitForFunctionArgs::new(expression);
        Self {
            inner,
            args,
            err: None
        }
    }

    pub async fn wait_for_function(self) -> Result<JsHandle, Arc<Error>> {
        let Self { inner, args, err } = self;
        if let Some(e) = err {
            return Err(e.into());
        }
        upgrade(&inner)?
            .wait_for_function(args)
            .await
            .map(JsHandle::new)
    }

    pub fn arg<T>(mut self, x: &T) -> Self
    where
        T: Serialize
    {
        let arg = match ser::to_value(x).map_err(Error::SerializationPwJson) {
            Err(e) => {
                self.err = Some(e);
                return self;
            }
            Ok(arg) => arg
        };
        self.args.arg = Some(arg);
        self
    }

    setter! {
        /// If `polling` is `'raf'`, then `expression` is constantly executed in `requestAnimationFrame` callback. If `polling` is a
        /// number, then it is treated as an interval in milliseconds at which the function would be executed. Defaults to `raf`.
        polling: Option<Polling>,
        timeout: Option<f64>
    }

    pub fn clear_arg(mut self) -> Self {
        self.args.arg = None;
        self.err = None;
        self
    }
}
