use playwright::{
    api::{Browser, BrowserContext, BrowserType, DateTime, Page, Response},
    *
};

runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    let mut playwright = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    playwright.prepare().unwrap(); // install browsers
    let mut chromium = playwright.chromium();
    let mut browser = chromium.launcher().headless(true).launch().await.unwrap();
    let mut context = browser.context_builder().build().await.unwrap();
    let mut page = context.new_page().await.unwrap();
    page.goto_builder("https://example.com/")
        .goto()
        .await
        .unwrap();
    let s: String = page.eval("() => location.href").await.unwrap();
    assert_eq!("https://example.com/", s);
    page.click_builder("a").click().await.unwrap();
});

runtime_test!(awesome, {
    env_logger::builder().is_test(true).try_init().ok();
    let (_playwright, _browser, _context, mut page) = init().await;
    let _response: Option<Response> = page
        .goto_builder("https://example.com/")
        .goto()
        .await
        .unwrap();
    let h = page.eval_handle("() => location.href").await.unwrap();
    let s: String = page
        .evaluate("([s]) => s + location.href", Some(vec![h]))
        .await
        .unwrap();
    assert_eq!(s, "https://example.com/https://example.com/");
    let s: DateTime = page
        .evaluate("d => d", Some(DateTime::from(chrono::Utc::now())))
        .await
        .unwrap();
    println!("{:?}", s);
    //// let _ = p.main_frame().query_selector_all("a").await.unwrap();
    //// let _ = p.main_frame().title().await.unwrap();
    // let mut a = p.query_selector("a").await.unwrap().unwrap();
    // let _href = a.get_attribute("href").await.unwrap();
    // dbg!(v);
    // p.go_back_builder().go_back().await.unwrap();
});

async fn init() -> (Playwright, Browser, BrowserContext, Page) {
    let pw = Playwright::initialize().await.unwrap();
    let mut b = launch(&mut pw.chromium()).await;
    let mut c = new_context(&mut b).await;
    let p = c.new_page().await.unwrap();
    (pw, b, c, p)
}

async fn launch(t: &mut BrowserType) -> Browser {
    t.launcher().headless(true).launch().await.unwrap()
}

async fn new_context(b: &mut Browser) -> BrowserContext {
    let a = "asdf".to_string();
    b.context_builder().user_agent(&a).build().await.unwrap()
}
