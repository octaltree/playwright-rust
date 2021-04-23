use crate::imp::{artifact::Artifact, core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Download {
    url: String,
    suggested_filename: String,
    artifact: Weak<Artifact>
}

impl Download {
    pub(crate) fn new(artifact: Weak<Artifact>, url: String, suggested_filename: String) -> Self {
        Self {
            url,
            suggested_filename,
            artifact
        }
    }

    pub(crate) fn url(&self) -> &str { &self.url }

    pub(crate) fn suggested_filename(&self) -> &str { &self.suggested_filename }

    pub(crate) async fn path(&self) -> ArcResult<Option<PathBuf>> {
        upgrade(&self.artifact)?.path_after_finished().await
    }

    pub(crate) async fn delete(&self) -> ArcResult<()> { upgrade(&self.artifact)?.delete().await }

    pub(crate) async fn save_as<P: AsRef<Path>>(&self, path: P) -> Result<(), Arc<Error>> {
        upgrade(&self.artifact)?.save_as(path).await
    }
}
