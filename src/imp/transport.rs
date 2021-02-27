use crate::imp::message;
use futures::{
    stream::Stream,
    task::{Context, Poll}
};
use std::{
    convert::TryInto,
    env, io,
    io::{Read, Write},
    pin::Pin,
    process::{ChildStdin, ChildStdout}
};
use thiserror::Error;

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
    const BUFSIZE: usize = 30000;

    pub(crate) fn try_new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Transport {
            stdin,
            stdout,
            length: None,
            buf: Vec::new()
        }
    }

    // TODO: guarantee ordering with que
    // pub(crate) async fn send(&mut self, req: &message::Request) -> Result<(), SendError> {
    //    let serialized = serde_json::to_vec(&req)?;
    //    log::debug!("SEND>{:?}", &serialized);
    //    let length = serialized.len() as u32;
    //    let mut bytes = length.to_le_bytes().to_vec();
    //    bytes.extend(serialized);
    //    self.stdin.write(&bytes)?;
    //    Ok(())
    //}
}

impl Stream for Transport {
    type Item = message::Response;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = self.get_mut();
        {
            let mut buf = [0; Self::BUFSIZE];
            let n = match this.stdout.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    log::error!("{:?}", e);
                    return Poll::Ready(None);
                }
            };
            this.buf.extend(&buf[..n]);
        }
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
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
                    Err(e) => {
                        log::error!("{:?}", e);
                        return Poll::Ready(None);
                    }
                    Ok(r) => r
                };
                log::debug!("MSG>{:?}", &msg);
                this.length = None;
                this.buf = this.buf[l..].to_owned();
                Poll::Ready(Some(msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::imp::driver::Driver;
    use futures::stream::StreamExt;
    use std::env;

    #[tokio::test]
    async fn tokio_read() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let mut conn = driver.run().await.unwrap();
        if let Some(x) = conn.transport.next().await {
            dbg!(x);
        }
    }

    #[async_std::test]
    async fn async_std_read() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let mut conn = driver.run().await.unwrap();
        if let Some(x) = conn.transport.next().await {
            dbg!(x);
        }
    }

    #[actix_rt::test]
    async fn actix_read() {
        env_logger::builder().is_test(true).try_init().ok();
        let tmp = env::temp_dir().join("playwright-rust-test/driver");
        let driver = Driver::try_new(&tmp).unwrap();
        let mut conn = driver.run().await.unwrap();
        if let Some(x) = conn.transport.next().await {
            dbg!(x);
        }
    }
}
