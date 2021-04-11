pub use crate::imp::page::AccessibilitySnapshoptResponse;
use crate::{
    api::ElementHandle,
    imp::{
        core::*,
        page::{AccessibilitySnapshoptArgs, Page as PageImpl},
        prelude::*
    }
};

#[derive(Debug)]
pub struct Accessibility {
    inner: Weak<PageImpl>
}

impl Accessibility {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    pub async fn snapshot_builder(&self) -> SnapshotBuilder {
        SnapshotBuilder::new(self.inner.clone())
    }
}

pub struct SnapshotBuilder {
    inner: Weak<PageImpl>,
    args: AccessibilitySnapshoptArgs
}

impl SnapshotBuilder {
    fn new(inner: Weak<PageImpl>) -> Self {
        let args = AccessibilitySnapshoptArgs::default();
        Self { inner, args }
    }

    pub async fn snapshot(self) -> ArcResult<Option<AccessibilitySnapshoptResponse>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.accessibility_snapshot(args).await
    }

    pub fn try_root(mut self, x: ElementHandle) -> Result<Self, Error> {
        let guid = x.guid()?;
        self.args.root = Some(OnlyGuid { guid });
        Ok(self)
    }

    optional_setter!(interesting_only, bool);

    pub fn clear_root(mut self) -> Self {
        self.args.root = None;
        self
    }
}
