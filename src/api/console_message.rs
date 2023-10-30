use crate::imp::{
    console_message::ConsoleMessage as Impl, core::*, prelude::*, utils::SourceLocation
};

/// `ConsoleMessage` objects are dispatched by page via the [page::Event::Console](crate::api::page::Event::Console) event.
#[derive(Clone)]
pub struct ConsoleMessage {
    inner: Weak<Impl>
}

impl ConsoleMessage {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    /// One of the following values: `'log'`, `'debug'`, `'info'`, `'error'`, `'warning'`, `'dir'`, `'dirxml'`, `'table'`,
    /// `'trace'`, `'clear'`, `'startGroup'`, `'startGroupCollapsed'`, `'endGroup'`, `'assert'`, `'profile'`, `'profileEnd'`,
    /// `'count'`, `'timeEnd'`.
    pub fn r#type(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.r#type().into()) }

    /// The text of the console message.
    pub fn text(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.text().into()) }

    /// URL of the resource followed by 0-based line and column numbers in the resource formatted as `URL:line:column`.
    pub fn location(&self) -> Result<SourceLocation, Error> {
        Ok(upgrade(&self.inner)?.location().to_owned())
    }
}
