use crate::imp::{core::*, prelude::*, video::Video as Impl};

#[derive(Debug)]
pub struct Video {
    inner: Impl
}

impl Video {
    pub(crate) fn new(inner: Impl) -> Self { Self { inner } }

    pub fn path(&self) -> Result<PathBuf, Error> { self.inner.path() }

    // doesn't work with this version
    async fn save_as<P: AsRef<Path>>(&self, path: P) -> ArcResult<()> {
        self.inner.save_as(path).await
    }

    // doesn't work with this version
    async fn delete(&self) -> ArcResult<()> { self.inner.delete().await }
}
