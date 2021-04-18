use super::Which;
use playwright::api::{BrowserContext, Page};

pub async fn all(c: &BrowserContext, p1: Page, which: Which) {
    let p2 = c.new_page().await.unwrap();
    assert_ne!(p1, p2);
    assert_eq!(&p1.context(), c);
    assert_eq!(&p2.context(), c);
}
