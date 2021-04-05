use crate::{
    api::JsHandle,
    imp::{console_message::ConsoleMessage as Impl, core::*, prelude::*, utils::SourceLocation}
};

pub struct ConsoleMessage {
    inner: Weak<Impl>
}

impl ConsoleMessage {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub fn r#type(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.r#type().into()) }

    pub fn text(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.text().into()) }

    pub fn location(&self) -> Result<SourceLocation, Error> {
        Ok(upgrade(&self.inner)?.location().to_owned())
    }

    pub fn args(&self) -> Result<Vec<JsHandle>, Error> {
        Ok(upgrade(&self.inner)?
            .args()
            .iter()
            .map(|x| JsHandle::new(x.clone()))
            .collect())
    }
}
