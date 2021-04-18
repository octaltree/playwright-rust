use super::Which;
use playwright::api::{BrowserContext, Page};

pub async fn all(c: &BrowserContext, _which: Which) -> Page {
    assert_eq!(c.browser().unwrap().is_some(), true);
    cookies_work(c).await;
    //
    set_timeout(c).await;
    pages(c).await
}

async fn pages(c: &BrowserContext) -> Page {
    let len = c.pages().unwrap().len();
    let page = c.new_page().await.unwrap();
    assert_eq!(c.pages().unwrap().len(), len + 1);
    page
}

async fn set_timeout(c: &BrowserContext) {
    c.set_default_navigation_timeout(10000).await.unwrap();
    c.set_default_timeout(10000).await.unwrap();
}

async fn cookies_work(c: &BrowserContext) {
    use playwright::api::Cookie;
    ensure_cookies_are_cleared(c).await;
    let cookie = Cookie {
        name: "foo".into(),
        value: "bar".into(),
        url: Some("https://example.com/".into()),
        domain: None,
        path: None,
        expires: None,
        http_only: None,
        secure: None,
        same_site: None
    };
    c.add_cookies(&[cookie.clone()]).await.unwrap();
    let cookies = c.cookies(&[]).await.unwrap();
    let first = cookies.into_iter().next().unwrap();
    assert_eq!(&first.name, "foo");
    assert_eq!(&first.value, "bar");
    ensure_cookies_are_cleared(c).await;
}

async fn ensure_cookies_are_cleared(c: &BrowserContext) {
    c.clear_cookies().await.unwrap();
    let cs = c.cookies(&[]).await.unwrap();
    assert_eq!(0, cs.len());
}
