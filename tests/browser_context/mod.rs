use super::Which;
use playwright::api::{browser::RecordVideo, Browser, BrowserContext, BrowserType};

pub async fn all(
    browser: &Browser,
    persistent: &BrowserContext,
    port: u16,
    _which: Which
) -> BrowserContext {
    let c = launch(browser).await;
    assert_ne!(persistent, &c);
    assert!(c.browser().unwrap().is_some());
    offline(&c, port).await;
    set_timeout(&c).await;
    cookies_should_work(&c).await;
    //
    add_init_script_should_work(&c).await;
    pages_should_work(&c).await;
    c
}

pub async fn persistent(t: &BrowserType, _port: u16, which: Which) -> BrowserContext {
    let c = launch_persistent_context(t).await;
    if Which::Firefox != which {
        // XXX: launch with permissions not work on firefox
        check_launched_permissions(&c).await;
    }
    c
}

async fn launch(b: &Browser) -> BrowserContext {
    let c = b
        .context_builder()
        .user_agent("asdf")
        .permissions(&["geolocation".into()])
        .accept_downloads(true)
        .has_touch(true)
        .record_video(RecordVideo {
            dir: &super::temp_dir().join("video"),
            size: None
        })
        .offline(true)
        .build()
        .await
        .unwrap();
    c.set_extra_http_headers(vec![("foo".into(), "bar".into())])
        .await
        .unwrap();
    c
}

async fn launch_persistent_context(t: &BrowserType) -> BrowserContext {
    t.persistent_context_launcher("./target".as_ref())
        .user_agent("asdf")
        .permissions(&["geolocation".into()])
        .launch()
        .await
        .unwrap()
}

async fn pages_should_work(c: &BrowserContext) {
    let len = c.pages().unwrap().len();
    let page = c.new_page().await.unwrap();
    assert_eq!(c.pages().unwrap().len(), len + 1);
    page.close(None).await.unwrap();
    page.close(None).await.unwrap();
    assert_eq!(c.pages().unwrap().len(), len);
}

async fn set_timeout(c: &BrowserContext) {
    c.set_default_navigation_timeout(10000).await.unwrap();
    c.set_default_timeout(10000).await.unwrap();
}

async fn cookies_should_work(c: &BrowserContext) {
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

async fn check_launched_permissions(c: &BrowserContext) {
    assert_eq!(get_permission(c, "geolocation").await, "granted");
    c.clear_permissions().await.unwrap();
    assert_eq!(get_permission(c, "geolocation").await, "prompt");
}

async fn get_permission(c: &BrowserContext, name: &str) -> String {
    let p = c.new_page().await.unwrap();
    let res = p
        .evaluate(
            "name => navigator.permissions.query({name}).then(result => result.state)",
            name
        )
        .await
        .unwrap();
    p.close(None).await.unwrap();
    res
}

async fn add_init_script_should_work(c: &BrowserContext) {
    c.add_init_script("HOGE = 2").await.unwrap();
    let p = c.new_page().await.unwrap();
    let x: i32 = p.eval("() => HOGE").await.unwrap();
    assert_eq!(x, 2);
    p.close(None).await.unwrap();
}

async fn offline(c: &BrowserContext, port: u16) {
    let page = c.new_page().await.unwrap();
    let url = super::url_static(port, "/empty.html");
    let err = page.goto_builder(&url).goto().await;
    assert!(err.is_err());
    c.set_offline(false).await.unwrap();
    let response = page.goto_builder(&url).goto().await.unwrap();
    assert_eq!(response.unwrap().status().unwrap(), 200);
}
