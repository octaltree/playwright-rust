use crate::imp::{core::*, prelude::*, selectors::Selectors as Impl};

/// Selectors can be used to install custom selector engines.
#[derive(Debug, Clone)]
pub struct Selectors {
    inner: Weak<Impl>
}

impl Selectors {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    /// An example of registering selector engine that queries elements based on a tag name:
    ///
    /// ```js
    /// const { selectors, firefox } = require('playwright');  // Or 'chromium' or 'webkit'.
    ///
    /// (async () => {
    ///  // Must be a function that evaluates to a selector engine instance.
    ///  const createTagNameEngine = () => ({
    ///    // Returns the first element matching given selector in the root's subtree.
    ///    query(root, selector) {
    ///      return root.querySelector(selector);
    ///    },
    ///
    ///    // Returns all elements matching given selector in the root's subtree.
    ///    queryAll(root, selector) {
    ///      return Array.from(root.querySelectorAll(selector));
    ///    }
    ///  });
    ///
    ///  // Register the engine. Selectors will be prefixed with "tag=".
    ///  await selectors.register('tag', createTagNameEngine);
    ///
    ///  const browser = await firefox.launch();
    ///  const page = await browser.newPage();
    ///  await page.setContent(`<div><button>Click me</button></div>`);
    ///
    ///  // Use the selector prefixed with its name.
    ///  const button = await page.$('tag=button');
    ///  // Combine it with other selector engines.
    ///  await page.click('tag=div >> text="Click me"');
    ///  // Can use it in any methods supporting selectors.
    ///  const buttonCount = await page.$$eval('tag=button', buttons => buttons.length);
    ///
    ///  await browser.close();
    /// })();
    /// ```
    /// # Args
    /// ## name
    /// Name that is used in selectors as a prefix, e.g. `{name: 'foo'}` enables `foo=myselectorbody` selectors.
    /// May only contain `[a-zA-Z0-9_]` characters.
    /// ## script
    /// Script that evaluates to a selector engine instance.
    /// ## content_script
    /// Whether to run this selector engine in isolated JavaScript environment. This environment
    /// has access to the same DOM, but not any JavaScript objects from the frame's scripts.
    /// Defaults to `false`. Note that running as a content script is not
    /// guaranteed when this engine is used together with other registered engines.
    pub async fn register(
        &self,
        name: &str,
        script: &str,
        content_script: bool
    ) -> Result<(), Arc<Error>> {
        let inner = upgrade(&self.inner)?;
        inner.register(name, script, content_script).await
    }
}
