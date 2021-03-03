use crate::imp::{core::*, prelude::*, selectors};

#[derive(Debug, Clone)]
pub struct Selectors {
    inner: Rweak<selectors::Selectors>
}

impl Selectors {
    pub(crate) fn new(inner: Rweak<selectors::Selectors>) -> Self { Self { inner } }
}
