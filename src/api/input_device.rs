use crate::imp::{
    core::*,
    page::{MouseClickArgs, Page as PageImpl},
    prelude::*,
    utils::MouseButton
};

/// Keyboard provides an api for managing a virtual keyboard. The high level api is [`method: Keyboard.type`], which takes
/// raw characters and generates proper keydown, keypress/input, and keyup events on your page.
///
/// For finer control, you can use [`method: Keyboard.down`], [`method: Keyboard.up`], and [`method: Keyboard.insertText`]
/// to manually fire events as if they were generated from a real keyboard.
///
/// An example of holding down `Shift` in order to select and delete some text:
///
/// ```js
/// await page.keyboard.type('Hello World!');
/// await page.keyboard.press('ArrowLeft');
///
/// await page.keyboard.down('Shift');
/// for (let i = 0; i < ' World'.length; i++)
///  await page.keyboard.press('ArrowLeft');
/// await page.keyboard.up('Shift');
///
/// await page.keyboard.press('Backspace');
///// Result text will end up saying 'Hello!'
/// ```
/// 
/// An example of pressing uppercase `A`
/// ```js
/// await page.keyboard.press('Shift+KeyA');
///// or
/// await page.keyboard.press('Shift+A');
/// ```
/// 
/// An example to trigger select-all with the keyboard
/// ```js
///// on Windows and Linux
/// await page.keyboard.press('Control+A');
///// on macOS
/// await page.keyboard.press('Meta+A');
/// ```
#[derive(Debug)]
pub struct Keyboard {
    inner: Weak<PageImpl>
}

/// The Mouse class operates in main-frame CSS pixels relative to the top-left corner of the viewport.
///
/// Every `page` object has its own Mouse, accessible with [`property: Page.mouse`].
///
/// ```js
///// Using ‘page.mouse’ to trace a 100x100 square.
/// await page.mouse.move(0, 0);
/// await page.mouse.down();
/// await page.mouse.move(0, 100);
/// await page.mouse.move(100, 100);
/// await page.mouse.move(100, 0);
/// await page.mouse.move(0, 0);
/// await page.mouse.up();
/// ```
#[derive(Debug)]
pub struct Mouse {
    inner: Weak<PageImpl>
}

/// The Touchscreen class operates in main-frame CSS pixels relative to the top-left corner of the viewport. Methods on the
/// touchscreen can only be used in browser contexts that have been initialized with `hasTouch` set to true.
#[derive(Debug)]
pub struct TouchScreen {
    inner: Weak<PageImpl>
}

impl Keyboard {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    /// Dispatches a `keydown` event.
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
    /// If `key` is a modifier key, `Shift`, `Meta`, `Control`, or `Alt`, subsequent key presses will be sent with that modifier
    /// active. To release the modifier key, use [`method: Keyboard.up`].
    ///
    /// After the key is pressed once, subsequent calls to [`method: Keyboard.down`] will have
    /// [repeat](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat) set to true. To release the key, use
    /// [`method: Keyboard.up`].
    ///
    /// > NOTE: Modifier keys DO influence `keyboard.down`. Holding down `Shift` will type the text in upper case.
    pub async fn down(&self, key: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_down(key).await
    }

    pub async fn up(&self, key: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_up(key).await
    }

    /// Dispatches only `input` event, does not emit the `keydown`, `keyup` or `keypress` events.
    ///
    /// ```js
    /// page.keyboard.insertText('嗨');
    /// ```
    ///
    ///
    /// > NOTE: Modifier keys DO NOT effect `keyboard.insertText`. Holding down `Shift` will not type the text in upper case.
    pub async fn input_text(&self, text: &str) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_input_text(text).await
    }

    /// > NOTE: Modifier keys DO NOT effect `keyboard.type`. Holding down `Shift` will not type the text in upper case.
    pub async fn r#type(&self, text: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_type(text, delay).await
    }

    /// Shortcut for [`method: Keyboard.down`] and [`method: Keyboard.up`].
    pub async fn press(&self, key: &str, delay: Option<f64>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.key_press(key, delay).await
    }
}

impl Mouse {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn r#move(&self, x: f64, y: f64, steps: Option<i32>) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_move(x, y, steps).await
    }

    pub async fn down(
        &self,
        button: Option<MouseButton>,
        click_count: Option<i32>
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_down(button, click_count).await
    }

    pub async fn up(
        &self,
        button: Option<MouseButton>,
        click_count: Option<i32>
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.mouse_up(button, click_count).await
    }

    /// Shortcut for [`method: Mouse.move`], [`method: Mouse.down`], [`method: Mouse.up`].
    pub fn click_builder(&self, x: f64, y: f64) -> ClickBuilder {
        ClickBuilder::new(self.inner.clone(), x, y)
    }

    /// Shortcut for [`method: Mouse.move`], [`method: Mouse.down`], [`method: Mouse.up`], [`method: Mouse.down`] and
    /// [`method: Mouse.up`].
    pub fn dblclick_builder(&self, x: f64, y: f64) -> DblClickBuilder {
        DblClickBuilder::new(self.inner.clone(), x, y)
    }
}

impl TouchScreen {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn tap(&self, x: f64, y: f64) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.screen_tap(x, y).await
    }
}

macro_rules! clicker {
    ($t: ident, $f: ident, $mf: ident) => {
        pub struct $t {
            inner: Weak<PageImpl>,
            args: MouseClickArgs
        }

        impl $t {
            pub(crate) fn new(inner: Weak<PageImpl>, x: f64, y: f64) -> Self {
                let args = MouseClickArgs::new(x, y);
                Self { inner, args }
            }

            pub async fn $f(self) -> Result<(), Arc<Error>> {
                let Self { inner, args } = self;
                let _ = upgrade(&inner)?.$mf(args).await?;
                Ok(())
            }

            setter! {
                /// Defaults to `left`.
                button: Option<MouseButton>,
                /// defaults to 1. See [UIEvent.detail].
                click_count: Option<i32>,
                /// Time to wait between `mousedown` and `mouseup` in milliseconds. Defaults to 0.
                delay: Option<f64>
            }
        }
    };
}

clicker!(ClickBuilder, click, mouse_click);
clicker!(DblClickBuilder, dblclick, mouse_dblclick);
