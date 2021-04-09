use crate::imp::{
    core::*,
    prelude::*,
    worker::{Evt, Worker as Impl}
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

    // pub fn url(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.url().to_owned()) }

    // pub fn is_closed(&self) -> Result<bool, Error> { Ok(upgrade(&self.inner)?.is_closed()) }
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
