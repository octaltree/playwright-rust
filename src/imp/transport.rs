use crate::imp::message;
use futures::{
    stream::Stream,
    task::{Context, Poll}
};
use std::{
    convert::TryInto,
    io,
    io::{Read, Write},
    pin::Pin
};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncWriteExt, ReadBuf},
    process::{ChildStdin, ChildStdout}
};

#[derive(Debug)]
pub(crate) struct Transport {
    stdin: ChildStdin,
    stdout: ChildStdout,
    length: Option<u32>,
    buf: Vec<u8>
}

#[derive(Error, Debug)]
pub enum SendError {
    #[error(transparent)]
    Serde(#[from] serde_json::error::Error),
    #[error(transparent)]
    Io(#[from] io::Error)
}

impl Transport {
    const BUFSIZE: usize = 10000;

    pub(crate) fn try_new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        // FIXME: spawn loop buffering
        Transport {
            stdin,
            stdout,
            length: None,
            buf: Vec::new()
        }
    }

    pub(crate) async fn send(&mut self, req: &message::Request<'_, '_>) -> Result<(), SendError> {
        let serialized = serde_json::to_vec(&req)?;
        log::debug!("SEND>{:?}", &serialized);
        let length = serialized.len() as u32;
        let mut bytes = length.to_le_bytes().to_vec();
        bytes.extend(serialized);
        self.stdin.write(&bytes).await?;
        Ok(())
    }
}

impl Stream for Transport {
    type Item = message::Response;

    // TODO: memory performance
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = self.get_mut();
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
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
                    log::debug!("RECV>{}", unsafe { std::str::from_utf8_unchecked(bytes) });
                    let msg: message::Response = match serde_json::from_slice(bytes) {
                        Err(e) => {
                            log::error!("{:?}", e);
                            return Poll::Ready(None);
                        }
                        Ok(r) => r
                    };
                    this.length = None;
                    this.buf = this.buf[l..].to_owned();
                    return Poll::Ready(Some(msg));
                }
            }
        }
        {
            let mut buf = [0; Self::BUFSIZE];
            let mut buf = ReadBuf::new(&mut buf);
            // FIXME: read前にバッファに残ってるのを返す バッファ処理してからreadする
            // TODO: error with async_std
            match Pin::new(&mut this.stdout).poll_read(cx, &mut buf) {
                Poll::Pending => pending!(),
                Poll::Ready(Ok(())) => {
                    this.buf.extend(buf.filled());
                }
                Poll::Ready(Err(e)) => {
                    log::error!("{:?}", e);
                    return Poll::Ready(None);
                }
            };
        }
        pending!();
    }
}

#[cfg(test)]
mod tests {
    use crate::imp::{driver::Driver, message::Request};
    use futures::stream::StreamExt;
    use serde_json::value::Value;
    use std::env;

    #[tokio::test]
    async fn tokio_read() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let conn = driver.run().await.unwrap();
        let c = &mut conn.lock().unwrap();
        let t = &mut c.transport;
        if let Some(x) = t.next().await {
            dbg!(x);
        }
    }

    #[actix_rt::test]
    async fn actix_read() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let conn = driver.run().await.unwrap();
        let c: &mut crate::imp::connection::Connection = &mut conn.lock().unwrap();
        let t = &mut c.transport;
        if let Some(x) = t.next().await {
            dbg!(x);
        }
    }

    #[actix_rt::test]
    async fn actix_write() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let conn = driver.run().await.unwrap();
        let c: &mut crate::imp::connection::Connection = &mut conn.lock().unwrap();
        let t = &mut c.transport;
        t.send(&Request {
            id: 1,
            guid: None,
            method: None,
            params: None
        })
        .await
        .unwrap();
    }
}
