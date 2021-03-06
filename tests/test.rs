use playwright::{
    api::{Browser, BrowserType},
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
    register_selector(&p).await;
    let mut bt = p.firefox();
    let mut b = launch(&mut bt).await;
});

async fn register_selector(p: &Playwright) {
    p.selectors().register("foo", "", false).await.unwrap();
}

async fn launch(t: &mut BrowserType) -> Browser {
    t.launcher().headless(true).launch().await.unwrap()
}
