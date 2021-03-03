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

    // TOOD: Ok Browser
    pub(crate) async fn launch(&self, args: LaunchArgs) -> Result<(), Rc<ConnectionError>> {
        let m: Str<Method> = "launch".to_owned().try_into().unwrap();
        let res = send_message!(self, m, args);
        Ok(())
    }
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

#[derive(Serialize)]
pub struct LaunchArgs {}
