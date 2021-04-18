use super::Which;
use playwright::api::{Browser, BrowserContext, BrowserType};

pub async fn all(t: BrowserType, which: Which) -> (Browser, BrowserContext) {
    name_should_work(&t, which);
    executable_should_exist(&t);
    should_handle_timeout(&t).await;
    should_fire_close(&t).await;
    should_be_callable_twice(&t).await;
    launch_close_browser(&t).await;
    tokio::join!(launch(&t), launch_persistent_context(&t))
}

async fn launch(t: &BrowserType) -> Browser {
    t.launcher()
        .headless(false)
        .clear_headless()
        .launch()
        .await
        .unwrap()
}

async fn launch_persistent_context(t: &BrowserType) -> BrowserContext {
    t.persistent_context_launcher("./target".as_ref())
        .launch()
        .await
        .unwrap()
}

async fn launch_close_browser(t: &BrowserType) {
    let (b1, b2) = tokio::join!(launch(&t), launch(&t));
    assert_ne!(b1, b2);
    b1.close().await.unwrap();
    b2.close().await.unwrap();
    assert_eq!(false, b1.exists());
}

fn name_should_work(t: &BrowserType, which: Which) {
    let name = t.name().unwrap();
    match which {
        Which::Webkit => assert_eq!(name, "webkit"),
        Which::Firefox => assert_eq!(name, "firefox"),
        Which::Chromium => assert_eq!(name, "chromium")
    }
}

fn executable_should_exist(t: &BrowserType) {
    let executable = t.executable().unwrap();
    assert_eq!(executable.is_file(), true);
}

// 'should handle timeout'
async fn should_handle_timeout(t: &BrowserType) {
    let result = t.launcher().timeout(0.1).launch().await;
    assert_eq!(result.is_err(), true);
    let err = result.err().unwrap();
    match &*err {
        playwright::Error::ErrorResponded(_) => {}
        e => {
            dbg!(e);
            unreachable!();
        }
    }
}

// 'should fire close event for all contexts'
async fn should_fire_close(t: &BrowserType) {
    use playwright::api::browser_context::{Event, EventType};
    let browser = t.launcher().launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let (wait, close) = tokio::join!(context.expect_event(EventType::Close), browser.close());
    close.unwrap();
    assert_eq!(wait.unwrap(), Event::Close);
}

// 'should be callable twice'
async fn should_be_callable_twice(t: &BrowserType) {
    let browser = t.launcher().launch().await.unwrap();
    let (fst, snd) = tokio::join!(browser.close(), browser.close());
    fst.unwrap();
    snd.unwrap();
}
