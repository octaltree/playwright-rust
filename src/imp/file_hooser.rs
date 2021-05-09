use crate::imp::{
    core::*, element_handle::ElementHandle as ElementHandleImpl, page::Page as PageImpl, prelude::*
};

/// `FileChooser` objects are dispatched by the page in the [page::Event::FileChooser](crate::api::page::Event::FileChooser) event.
///
/// ```js
/// const [fileChooser] = await Promise.all([
///  page.waitForEvent('filechooser'),
///  page.click('upload')
/// ]);
/// await fileChooser.setFiles('myfile.pdf');
/// ```
#[derive(Debug, Clone)]
pub struct FileChooser {
    pub(crate) page: Weak<PageImpl>,
    pub(crate) element_handle: Weak<ElementHandleImpl>,
    pub(crate) is_multiple: bool
}

impl FileChooser {
    pub(crate) fn new(
        page: Weak<PageImpl>,
        element_handle: Weak<ElementHandleImpl>,
        is_multiple: bool
    ) -> Self {
        Self {
            page,
            element_handle,
            is_multiple
        }
    }
}
