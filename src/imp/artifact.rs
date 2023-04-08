use crate::imp::{core::*, prelude::*};

#[derive(Debug)]
pub(crate) struct Artifact {
    channel: ChannelOwner,
    pub(crate) absolute_path: String,
    var: Mutex<Variable>
}

#[derive(Debug, Default)]
pub(crate) struct Variable {
    is_remote: bool
}

impl Artifact {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { absolute_path } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            absolute_path,
            var: Mutex::default()
        })
    }

    pub(crate) async fn path_after_finished(&self) -> ArcResult<Option<PathBuf>> {
        if self.is_remote() {
            return Err(Error::RemoteArtifact.into());
        }
        let v = send_message!(self, "pathAfterFinished", Map::new());
        let p: Option<PathBuf> = maybe_only_str(&*v)?.map(|s| s.into());
        Ok(p)
    }

    pub(crate) async fn delete(&self) -> ArcResult<()> {
        let _ = send_message!(self, "delete", Map::new());
        Ok(())
    }

    pub(crate) async fn save_as<P: AsRef<Path>>(&self, path: P) -> ArcResult<()> {
        let path = path.as_ref();
        let dir = path
            .parent()
            .ok_or_else(|| Error::ResolvePath(path.into()))?;
        let res = send_message!(self, "saveAsStream", Map::new());
        let guid = only_guid(&res)?;
        let stream = get_object!(self.context()?.lock(), guid, Stream)?;
        std::fs::create_dir_all(dir).map_err(Error::from)?;
        upgrade(&stream)?.save_as(path).await?;
        Ok(())
    }

    pub(crate) async fn failure(&self) -> ArcResult<Option<String>> {
        let v = send_message!(self, "failure", Map::new());
        let msg = maybe_only_str(&v)?;
        Ok(msg.map(ToOwned::to_owned))
    }
}

// mutable
impl Artifact {
    fn set_is_remote(&self, x: bool) { self.var.lock().is_remote = x; }

    fn is_remote(&self) -> bool { self.var.lock().is_remote }
}

impl RemoteObject for Artifact {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    absolute_path: String
}
