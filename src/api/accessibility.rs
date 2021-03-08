use crate::imp::{page::Page as PageImpl, prelude::*};

pub struct Accessibility {
    inner: Weak<PageImpl>
}

impl Accessibility {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }
}
