pub use crate::imp::page::{AccessibilitySnapshotResponse as SnapshotResponse, Mixed, Val};
use crate::{
    api::ElementHandle,
    imp::{
        core::*,
        page::{AccessibilitySnapshotArgs as SnapshotArgs, Page as PageImpl},
        prelude::*
    }
};

/// The Accessibility class provides methods for inspecting Chromium's accessibility tree. The accessibility tree is used by
/// assistive technology such as [screen readers](https://en.wikipedia.org/wiki/Screen_reader) or
/// [switches](https://en.wikipedia.org/wiki/Switch_access).
///
/// Accessibility is a very platform-specific thing. On different platforms, there are different screen readers that might
/// have wildly different output.
///
/// Rendering engines of Chromium, Firefox and WebKit have a concept of "accessibility tree", which is then translated into
/// different platform-specific APIs. Accessibility namespace gives access to this Accessibility Tree.
///
/// Most of the accessibility tree gets filtered out when converting from internal browser AX Tree to Platform-specific
/// AX-Tree or by assistive technologies themselves. By default, Playwright tries to approximate this filtering, exposing
/// only the "interesting" nodes of the tree.
#[derive(Debug)]
pub struct Accessibility {
    inner: Weak<PageImpl>
}

impl Accessibility {
    pub(crate) fn new(inner: Weak<PageImpl>) -> Self { Self { inner } }

    /// Captures the current state of the accessibility tree. The returned object represents the root accessible node of the
    /// page.
    ///
    /// > NOTE: The Chromium accessibility tree contains nodes that go unused on most platforms and by most screen readers.
    /// Playwright will discard them as well for an easier to process tree, unless `interestingOnly` is set to `false`.
    ///
    /// An example of dumping the entire accessibility tree:
    ///
    /// ```js
    /// const snapshot = await page.accessibility.snapshot();
    /// console.log(snapshot);
    /// ```
    ///
    /// An example of logging the focused node's name:
    ///
    /// ```js
    /// const snapshot = await page.accessibility.snapshot();
    /// const node = findFocusedNode(snapshot);
    /// console.log(node && node.name);
    ///
    /// function findFocusedNode(node) {
    ///  if (node.focused)
    ///    return node;
    ///  for (const child of node.children || []) {
    ///    const foundNode = findFocusedNode(child);
    ///    return foundNode;
    ///  }
    ///  return null;
    /// }
    /// var accessibilitySnapshot = await Page.Accessibility.SnapshotAsync();
    /// var focusedNode = findFocusedNode(accessibilitySnapshot);
    /// if(focusedNode != null)
    ///  Console.WriteLine(focusedNode.Name);
    /// ```
    pub fn snapshot_builder(&self) -> SnapshotBuilder { SnapshotBuilder::new(self.inner.clone()) }
}

pub struct SnapshotBuilder {
    inner: Weak<PageImpl>,
    args: SnapshotArgs
}

impl SnapshotBuilder {
    fn new(inner: Weak<PageImpl>) -> Self {
        let args = SnapshotArgs::default();
        Self { inner, args }
    }

    pub async fn snapshot(self) -> ArcResult<Option<SnapshotResponse>> {
        let Self { inner, args } = self;
        upgrade(&inner)?.accessibility_snapshot(args).await
    }

    /// The root DOM element for the snapshot. Defaults to the whole page.
    pub fn try_root(mut self, x: ElementHandle) -> Result<Self, Error> {
        let guid = x.guid()?;
        self.args.root = Some(OnlyGuid { guid });
        Ok(self)
    }

    setter!(
        /// Prune uninteresting nodes from the tree. Defaults to `true`.
        interesting_only: Option<bool>
    );

    pub fn clear_root(mut self) -> Self {
        self.args.root = None;
        self
    }
}
