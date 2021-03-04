use playwright::*;

runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
    let chromium = p.chromium();
});

runtime_test!(awesome, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
    register_selector(&p).await;
    // let chromium = p.chromium();
});

async fn register_selector(p: &Playwright) {
    p.selectors().register("foo", "", false).await.unwrap();
}
