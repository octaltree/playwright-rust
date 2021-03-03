use crate::imp::{self, core::*, prelude::*};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Rweak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Rweak<imp::browser_type::BrowserType>) -> Self { Self { inner } }
}
