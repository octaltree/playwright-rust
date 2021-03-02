use crate::imp::{core::*, prelude::*};
use futures::task::{Context, Poll};
use std::{future::Future, pin::Pin, sync::TryLockError};

#[derive(Debug)]
pub(crate) struct Selectors {
    channel: ChannelOwner
}

impl Selectors {
    pub(crate) fn new(channel: ChannelOwner) -> Self { Self { channel } }

    // TODO: 送信前エラー, 送信後エラーまとめる
    pub(crate) fn register(
        &self,
        name: &str,
        script: &str,
        is_content_script: bool
    ) -> Result<WaitMessage, ConnectionError> {
        let mut p = Map::<String, Value>::default();
        p.insert("name".into(), name.into());
        p.insert("script".into(), script.into());
        p.insert("contentScript".into(), is_content_script.into());
        let (w, weak) = WaitMessage::new();
        let r = self
            .channel()
            .create_request("register".to_owned().try_into().unwrap())
            .set_params(p)
            .set_place(weak);
        self.channel()
            .tx
            .unbounded_send(r)
            .map_err(|_| ConnectionError::Channel)?;
        Ok(w)
    }
}

pub(crate) struct WaitMessage {
    // FIXME: Option<Result<ResponseResult, ConnectionError>>
    placeholder: Rc<Mutex<Option<ResponseResult>>>
}

impl WaitMessage {
    fn new() -> (Self, Weak<Mutex<Option<ResponseResult>>>) {
        let placeholder = Rc::new(Mutex::new(None));
        let weak = Rc::downgrade(&placeholder);
        (WaitMessage { placeholder }, weak)
    }
}

impl Future for WaitMessage {
    type Output = ResponseResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        macro_rules! pending {
            () => {{
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }};
        }
        let x = match this.placeholder.try_lock() {
            Ok(x) => x,
            Err(TryLockError::WouldBlock) => pending!(),
            Err(e) => Err(e).unwrap()
        };
        if let Some(x) = &*x {
            return Poll::Ready(x.clone());
        } else {
            pending!()
        }
    }
}

impl RemoteObject for Selectors {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
