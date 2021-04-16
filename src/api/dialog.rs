use crate::imp::{core::*, dialog::Dialog as Impl, prelude::*};

/// `Dialog` objects are dispatched by page via the [page::Event::Dialog](crate::api::page::Event::Dialog) event.
///
/// An example of using `Dialog` class:
///
/// ```js
/// const { chromium } = require('playwright');  // Or 'firefox' or 'webkit'.
///
/// (async () => {
///  const browser = await chromium.launch();
///  const page = await browser.newPage();
///  page.on('dialog', async dialog => {
///    console.log(dialog.message());
///    await dialog.dismiss();
///  });
///  await page.evaluate(() => alert('1'));
///  await browser.close();
/// })();
/// ```
///
/// > NOTE: Dialogs are dismissed automatically, unless there is a [`event: Page.dialog`] listener. When listener is
/// present, it **must** either [`method: Dialog.accept`] or [`method: Dialog.dismiss`] the dialog - otherwise the page will
/// [freeze](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop#never_blocking) waiting for the dialog, and
/// actions like click will never finish.
pub struct Dialog {
    inner: Weak<Impl>
}

impl Dialog {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    ///// Returns when the dialog has been accepted.
    ///// A text to enter in prompt. Does not cause any effects if the dialog's `type` is not prompt. Optional.
    // fn accept(&self, prompt_text: Option<String>) -> Result<(), Arc<Error>> { todo!() }
    ///// If dialog is prompt, returns default prompt value. Otherwise, returns empty string.
    // fn default_value(&self) -> Result<String, Error> { todo!() }
    ///// Returns when the dialog has been dismissed.
    // fn dismiss(&self) -> Result<(), Arc<Error>> { todo!() }
    ///// A message displayed in the dialog.
    // fn message(&self) -> Result<String, Error> { todo!() }
    ///// Returns dialog's type, can be one of `alert`, `beforeunload`, `confirm` or `prompt`.
    // fn r#type(&self) -> Result<String, Error> { todo!() }
}
