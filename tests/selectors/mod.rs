use super::Which;
use playwright::api::{Playwright, Selectors};

pub async fn all(playwright: &Playwright, which: Which) {
    let selectors = playwright.selectors();

    register_should_work(playwright, &selectors, which).await;
}

async fn register_should_work(playwright: &Playwright, selectors: &Selectors, which: Which) {
    let snip = "({
        // Returns the first element matching given selector in the root's subtree.
        query(root, selector) {
          return root.querySelector(selector);
        },

        // Returns all elements matching given selector in the root's subtree.
        queryAll(root, selector) {
          return Array.from(root.querySelectorAll(selector));
        }
      })";
    selectors.register("tag", snip, false).await.unwrap();
    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium()
    };
    let browser = t.launcher().launch().await.unwrap();
    let bc = browser.context_builder().build().await.unwrap();
    let page = bc.new_page().await.unwrap();
    page.set_content_builder("<div><button>Click me</button></div>")
        .set_content()
        .await
        .unwrap();
    let _button = page.query_selector("tag=button").await.unwrap().unwrap();
    page.click_builder(r#"tag=div >> text="Click me""#)
        .click()
        .await
        .unwrap();
}
