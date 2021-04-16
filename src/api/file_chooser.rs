use crate::imp::{core::*, prelude::*};

/// `FileChooser` objects are dispatched by the page in the [page::Event::FileChooser](crate::api::page::Event::FileChooser) event.
///
/// ```js
/// const [fileChooser] = await Promise.all([
///  page.waitForEvent('filechooser'),
///  page.click('upload')
/// ]);
/// await fileChooser.setFiles('myfile.pdf');
/// ```
pub struct FileChooser {}

impl FileChooser {
    ///// Returns input element associated with this file chooser.
    // fn element(&self) -> Result<ElementHandle, Error> { todo!() }
    ///// Returns whether this file chooser accepts multiple files.
    // fn is_multiple(&self) -> Result<bool, Error> { todo!() }
    ///// Returns page this file chooser belongs to.
    // fn page(&self) -> Result<Page, Error> { todo!() }
    ///// Sets the value of the file input this chooser is associated with. If some of the `filePaths` are relative paths, then
    ///// they are resolved relative to the the current working directory. For empty array, clears the selected files.
    // fn set_files(
    //    &self,
    //    /// files: NotImplementedYet,
    //    /// options
    //    /// Actions that initiate navigations are waiting for these navigations to happen and for pages to start loading. You can
    //    /// opt out of waiting via setting this flag. You would only need this option in the exceptional cases such as navigating to
    //    /// inaccessible pages. Defaults to `false`.
    //    no_wait_after: Option<bool>,
    //    /// Maximum time in milliseconds, defaults to 30 seconds, pass `0` to disable timeout. The default value can be changed by
    //    /// using the [`method: BrowserContext.setDefaultTimeout`] or [`method: Page.setDefaultTimeout`] methods.
    //    timeout: Option<f64>
    //) -> Result<(), Arc<Error>> {
    //    todo!()
    //}
}
