use crate::imp::{core::*, js_handle::JsHandle as Impl, prelude::*};

pub struct JsHandle {
    inner: Weak<Impl>
}

impl JsHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }
}
