use super::Which;
use playwright::api::{Browser, BrowserType};

pub async fn all(t: BrowserType, which: Which) -> Browser {
    name_should_work(&t, which);
    executable_should_exist(&t);
    should_handle_timeout(&t).await;
    // should_fire_close(&t).await;
    t.launcher().launch().await.unwrap()
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
    let wait = context.expect_event(EventType::Close);
    browser.close().await.unwrap();
    assert_eq!(wait.await.unwrap(), Event::Close);
}
