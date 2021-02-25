use crate::imp::transport::Transport;
use std::{
    io,
    path::Path,
    process::{Child, Command, Stdio}
};

pub(crate) struct Connection {
    child: Child,
    transport: Transport
}

impl Connection {
    pub(crate) fn try_new(exec: &Path) -> io::Result<Connection> {
        let mut child = Command::new(exec)
            .args(&["run-driver"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let transport = Transport::try_new(stdin, stdout);
        Ok(Connection { child, transport })
    }
}
