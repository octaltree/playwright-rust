use crate::imp::core::*;
use std::{
    convert::TryInto,
    io,
    io::{Read, Write},
    process::{ChildStdin, ChildStdout}
};
use thiserror::Error;

#[derive(Debug)]
pub(super) struct Reader {
    stdout: ChildStdout,
    length: Option<u32>,
    buf: Vec<u8>
}

#[derive(Debug)]
pub(super) struct Writer {
    stdin: ChildStdin
}

#[derive(Error, Debug)]
pub enum TransportError {
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),
    #[error(transparent)]
    Io(#[from] io::Error)
}

impl Reader {
    const BUFSIZE: usize = 32767;

    pub(super) fn new(stdout: ChildStdout) -> Self {
        Self {
            stdout,
            length: None,
            buf: Vec::with_capacity(Self::BUFSIZE)
        }
    }

    // TODO: heap efficiency
    pub(super) fn try_read(&mut self) -> Result<Option<Res>, TransportError> {
        let this = self;
        {
            if this.length.is_none() && this.buf.len() >= 4 {
                let off = this.buf.split_off(4);
                let bytes: &[u8] = &this.buf;
                this.length = Some(u32::from_le_bytes(bytes.try_into().unwrap()));
                this.buf = off;
            }
            match this.length.map(|u| u as usize) {
                None => {}
                Some(l) if this.buf.len() < l => {}
                Some(l) => {
                    let bytes: &[u8] = &this.buf[..l];
                    log::debug!("RECV {}", unsafe { std::str::from_utf8_unchecked(bytes) });
                    let msg: Res = serde_json::from_slice(bytes)?;
                    this.length = None;
                    this.buf = this.buf[l..].to_owned();
                    return Ok(Some(msg));
                }
            }
        }
        {
            let mut buf = [0; Self::BUFSIZE];
            let n = this.stdout.read(&mut buf)?;
            this.buf.extend(&buf[..n]);
        }
        Ok(None)
    }
}

impl Writer {
    pub(super) fn new(stdin: ChildStdin) -> Self { Self { stdin } }

    pub(super) fn send(&mut self, req: &Req<'_, '_>) -> Result<(), TransportError> {
        log::debug!("SEND {:?}", &req);
        let serialized = serde_json::to_vec(&req)?;
        let length = serialized.len() as u32;
        let mut bytes = length.to_le_bytes().to_vec();
        bytes.extend(serialized);
        self.stdin.write_all(&bytes)?;
        Ok(())
    }
}
