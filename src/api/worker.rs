use crate::{
    api::JsHandle,
    imp::{
        core::*,
        prelude::*,
        worker::{Evt, Worker as Impl}
    }
};

pub struct Worker {
    inner: Weak<Impl>
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl Worker {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }

    pub async fn eval_handle(&self, expression: &str) -> ArcResult<JsHandle> {
        upgrade(&self.inner)?
            .eval_handle(expression)
            .await
            .map(JsHandle::new)
    }

    pub async fn evaluate_handle<T>(&self, expression: &str, arg: Option<T>) -> ArcResult<JsHandle>
    where
        T: Serialize
    {
        upgrade(&self.inner)?
            .evaluate_handle(expression, arg)
            .await
            .map(JsHandle::new)
    }

    pub async fn eval<U>(&self, expression: &str) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.eval(expression).await
    }

    pub async fn evaluate<T, U>(&self, expression: &str, arg: Option<T>) -> ArcResult<U>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.evaluate(expression, arg).await
    }
}

#[derive(Debug)]
pub(crate) enum Event {
    Close
}

impl From<Evt> for Event {
    fn from(e: Evt) -> Self {
        match e {
            Evt::Close => Self::Close
        }
    }
}
