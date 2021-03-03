use crate::imp::{core::*, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct BrowserContext {
    channel: ChannelOwner
}

impl RemoteObject for BrowserContext {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
