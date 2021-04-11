use super::Which;
use playwright::api::{BrowserContext, Page};

pub async fn all(c: BrowserContext, _which: Which) -> Page {
    assert_eq!(c.browser().unwrap().is_some(), true);
    // unimplemented!()
    pages(&c).await
}

async fn pages(c: &BrowserContext) -> Page {
    let len = c.pages().unwrap().len();
    let page = c.new_page().await.unwrap();
    assert_eq!(c.pages().unwrap().len(), len + 1);
    page
}
