use crate::imp::{core::*, element_handle::ElementHandle as Impl, prelude::*};

pub struct ElementHandle {
    inner: Weak<Impl>
}

impl ElementHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }
}
