use playwright::*;

#[tokio::test]
async fn hello() {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap();
    p.prepare().unwrap();
}
