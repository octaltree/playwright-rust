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

pub(crate) struct Transport {
    stdin: ChildStdin,
    stdout: ChildStdout,
    length: Option<u32>,
    buf: Vec<u8>
}

impl Transport {
    const BUFSIZE: usize = 1000;

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

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = self.get_mut();
        let mut buf = [0; Self::BUFSIZE];
        let n = match this.stdout.read(&mut buf) {
            Ok(n) => n,
            Err(_) => return Poll::Ready(None)
        };
        dbg!(n);
        this.buf.extend(&buf);
        if this.length.is_none() {
            if this.buf.len() >= 4 {
                let off = this.buf.split_off(4);
                let bytes: &[u8] = &this.buf;
                this.length = Some(u32::from_le_bytes(bytes.try_into().unwrap()));
                this.buf = off;
            } else {
                return Poll::Pending;
            }
        }
        match this.length.map(|u| u as usize) {
            None => Poll::Pending,
            Some(l) if this.buf.len() < l => Poll::Pending,
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
    use futures::{stream, stream::StreamExt};

    #[cfg(feature = "runtime-async-std")]
    #[async_std::test]
    async fn async_std_stream() {
        let mut stream = stream::iter(0..5);
        while let Some(item) = stream.next().await {}
    }

    #[cfg(feature = "runtime-actix")]
    #[actix_rt::test]
    async fn actix_rt_stream() {
        let mut stream = stream::iter(0..5);
        while let Some(item) = stream.next().await {}
    }

    #[cfg(feature = "runtime-tokio")]
    #[tokio::test]
    async fn tokio_stream() {
        let mut stream = stream::iter(0..5);
        while let Some(item) = stream.next().await {}
    }
}
