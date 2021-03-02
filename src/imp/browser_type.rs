use crate::imp::{core::*, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct BrowserType {
    channel: ChannelOwner,
    name: String,
    executable: PathBuf
}

impl BrowserType {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, ConnectionError> {
        let Initializer { name, executable } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            name,
            executable
        })
    }

    pub(crate) fn name(&self) -> &str { &self.name }

    pub(crate) fn executable(&self) -> &Path { &self.executable }
}

impl RemoteObject for BrowserType {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    name: String,
    #[serde(rename = "executablePath")]
    executable: PathBuf
}
