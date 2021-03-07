use playwright::{
    api::{Browser, BrowserContext, BrowserType, DocumentLoadState, Response},
    *
};

runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
});

runtime_test!(awesome, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
    let mut bt = p.chromium();
    let mut b = launch(&mut bt).await;
    let mut c = new_context(&mut b).await;
    let mut p = c.new_page().await.unwrap();
    let _response: Option<Response> = p
        .main_frame()
        .goto_builder("https://example.com/")
        //.wait_until(DocumentLoadState::Load)
        .goto()
        .await
        .unwrap();
    p.clicker("a").click().await.unwrap();
    p.go_back_builder().go_back().await.unwrap();
});

async fn launch(t: &mut BrowserType) -> Browser {
    t.launcher().headless(true).launch().await.unwrap()
}

async fn new_context(b: &mut Browser) -> BrowserContext {
    let a = "asdf".to_string();
    b.context_builder().user_agent(&a).build().await.unwrap()
}
