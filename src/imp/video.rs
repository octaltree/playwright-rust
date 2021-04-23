use crate::imp::{artifact::Artifact, core::*, prelude::*};

#[derive(Debug, Clone)]
pub(crate) struct Video {
    artifact: Weak<Artifact>
}

impl Video {
    pub(crate) fn new(artifact: Weak<Artifact>) -> Self { Self { artifact } }

    pub(crate) fn path(&self) -> Result<PathBuf, Error> {
        Ok(upgrade(&self.artifact)?.absolute_path.as_str().into())
    }

    pub(crate) async fn save_as<P: AsRef<Path>>(&self, path: P) -> ArcResult<()> {
        upgrade(&self.artifact)?.save_as(path).await
    }

    pub(crate) async fn delete(&self) -> ArcResult<()> { upgrade(&self.artifact)?.delete().await }
}
