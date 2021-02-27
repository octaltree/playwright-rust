use crate::imp::message;
use futures::{
    stream::Stream,
    task::{Context, Poll}
};
use std::{
    collections::VecDeque,
    convert::TryInto,
    env, io,
    io::Read,
    pin::Pin,
    process::{ChildStdin, ChildStdout}
};

#[derive(Debug)]
pub(crate) struct Transport {
    stdin: ChildStdin,
    stdout: ChildStdout,
    length: Option<u32>,
    buf: Vec<u8>
}

impl Transport {
    const BUFSIZE: usize = 30000;

    pub(crate) fn try_new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Transport {
            stdin,
            stdout,
            length: None,
            buf: Vec::new()
        }
    }
}

impl Stream for Transport {
    type Item = message::Response;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = self.get_mut();
        let mut buf = [0; Self::BUFSIZE];
        let n = match this.stdout.read(&mut buf) {
            Ok(n) => n,
            Err(_) => return Poll::Ready(None)
        };
        dbg!(n);
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
        this.buf.extend(&buf[..n]);
        if this.length.is_none() {
            if this.buf.len() >= 4 {
                let off = this.buf.split_off(4);
                let bytes: &[u8] = &this.buf;
                this.length = Some(u32::from_le_bytes(bytes.try_into().unwrap()));
                this.buf = off;
            } else {
                // TODO: Is it needed waiting for wake if len==0?
                pending!()
            }
        }
        match this.length.map(|u| u as usize) {
            None => pending!(),
            Some(l) if this.buf.len() < l => pending!(),
            Some(l) => {
                let bytes: &[u8] = &this.buf[..l];
                log::debug!("RECV>{:?}", unsafe { std::str::from_utf8_unchecked(bytes) });
                let msg: message::Response = match serde_json::from_slice(bytes) {
                    Err(_) => return Poll::Ready(None),
                    Ok(r) => r
                };
                Poll::Ready(Some(msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
