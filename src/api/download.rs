use crate::imp::{core::*, download::Download as Impl, prelude::*};

/// `Download` objects are dispatched by page via the [`event: Page.download`] event.
///
/// All the downloaded files belonging to the browser context are deleted when the browser context is closed. All downloaded
/// files are deleted when the browser closes.
///
/// Download event is emitted once the download starts. Download path becomes available once download completes:
///
/// ```js
/// const [ download ] = await Promise.all([
///  page.waitForEvent('download'), // wait for download to start
///  page.click('a')
/// ]);
///// wait for download to complete
/// const path = await download.path();
/// ```
/// 
/// > NOTE: Browser context **must** be created with the `acceptDownloads` set to `true` when user needs access to the
/// downloaded content. If `acceptDownloads` is not set, download events are emitted, but the actual download is not
/// performed and user has no access to the downloaded files.
pub struct Download {
    inner: Arc<Impl>
}

impl Download {
    pub(crate) fn new(inner: Arc<Impl>) -> Self { Self { inner } }

    /// Returns downloaded url.
    pub fn url(&self) -> &str { self.inner.url() }

    /// Returns suggested filename for this download. It is typically computed by the browser from the
    /// [`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) response header
    /// or the `download` attribute. See the spec on [whatwg](https://html.spec.whatwg.org/#downloading-resources). Different
    /// browsers can use different logic for computing it.
    pub fn suggested_filename(&self) -> &str { self.inner.suggested_filename() }

    /// Returns path to the downloaded file in case of successful download. The method will wait for the download to finish if
    /// necessary.
    pub async fn path(&self) -> ArcResult<Option<PathBuf>> { self.inner.path().await }

    /// Deletes the downloaded file. Will wait for the download to finish if necessary.
    pub async fn delete(&self) -> ArcResult<()> { self.inner.delete().await }

    /// Saves the download to a user-specified path. It is safe to call this method while the download is still in progress.
    /// Path where the download should be saved.
    pub async fn save_as<P: AsRef<Path>>(&self, path: P) -> Result<(), Arc<Error>> {
        self.inner.save_as(path).await
    }

    ///// Returns readable stream for current download or `null` if download failed.
    // fn create_read_stream(&self) -> Result<Option<Readable>, Arc<Error>> { todo!() }

    /// Returns download error if any. Will wait for the download to finish if necessary.
    pub async fn failure(&self) -> Result<Option<String>, Arc<Error>> { self.inner.failure().await }
}
