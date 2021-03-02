use crate::imp::{core::*, prelude::*};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct BrowserType {
    channel: ChannelOwner
}

impl BrowserType {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

    pub(crate) fn name(&self) -> Option<&str> {
        let o = self.channel().initializer.as_object()?;
        o.get("name")?.as_str()
    }

    pub(crate) fn executable(&self) -> Option<&Path> {
        let o = self.channel().initializer.as_object()?;
        let s = o.get("executablePath")?.as_str()?;
        let p: &Path = s.as_ref();
        Some(p)
    }
}

impl RemoteObject for BrowserType {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
