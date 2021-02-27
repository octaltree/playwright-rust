use crate::imp::transport::Transport;
use futures::stream::StreamExt;
use std::{
    io,
    path::Path,
    process::{Child, Command, Stdio}
};

pub(crate) struct Connection {
    child: Child,
    pub(crate) transport: Transport
}

pub(crate) trait ChannelOwner {}

impl Connection {
    pub(crate) async fn try_new(exec: &Path) -> io::Result<Connection> {
        let mut child = Command::new(exec)
            .args(&["run-driver"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        // TODO: env "NODE_OPTIONS"
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let transport = Transport::try_new(stdin, stdout);
        Ok(Connection { child, transport })
    }

    pub(crate) async fn receive_initializer_message(&mut self) {
        while let Some(x) = self.transport.next().await {}
    }
}

#[cfg(test)]
mod tests {
    use crate::imp::driver::Driver;
    use std::env;

    crate::runtime_test!(try_new, {
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let _conn = driver.run().await.unwrap();
    });
}
