use crate::imp::{core::*, prelude::*};
use std::{
    fs::File,
    io::{BufWriter, Write}
};
use base64::Engine;

#[derive(Debug)]
pub(crate) struct Stream {
    channel: ChannelOwner
}

impl Stream {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

    pub(crate) async fn save_as<P: AsRef<Path>>(&self, path: P) -> ArcResult<()> {
        let file = File::create(path).map_err(Error::from)?;
        let mut writer = BufWriter::new(file);
        loop {
            let v = send_message!(self, "read", Map::new());
            let b64 = only_str(&v)?;
            if b64.is_empty() {
                break;
            } else {
                let bytes = base64::engine::general_purpose::STANDARD.decode(b64).map_err(Error::InvalidBase64)?;
                writer.write(&bytes).map_err(Error::from)?;
            }
        }
        Ok(())
    }

    // with open(path, mode="wb") as file:
    //    while True:
    //        binary = await self._channel.send("read")
    //        if not binary:
    //            break
    //        file.write(base64.b64decode(binary))
}

impl RemoteObject for Stream {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
