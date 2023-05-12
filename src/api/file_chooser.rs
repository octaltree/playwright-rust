pub use crate::imp::file_hooser::FileChooser;
use crate::{
    api::{element_handle::SetInputFilesBuilder, ElementHandle, Page},
    imp::utils::File
};

impl FileChooser {
    /// Returns input element associated with this file chooser.
    pub fn element(&self) -> ElementHandle { ElementHandle::new(self.element_handle.clone()) }
    /// Returns whether this file chooser accepts multiple files.
    pub fn is_multiple(&self) -> bool { self.is_multiple }
    /// Returns page this file chooser belongs to.
    pub fn page(&self) -> Page { Page::new(self.page.clone()) }

    /// Sets the value of the file input this chooser is associated with. If some of the `filePaths` are relative paths, then
    /// they are resolved relative to the the current working directory. For empty array, clears the selected files.
    pub fn set_input_files_builder(&self, file: File) -> SetInputFilesBuilder {
        SetInputFilesBuilder::new(self.element_handle.clone(), file)
    }
}
