use super::Which;
use playwright::api::{BrowserType, Playwright};

pub async fn all(playwright: &Playwright, which: Which) -> BrowserType {
    let t = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium()
    };
    name_should_work(&t, which);
    executable_should_exist(&t);
    should_handle_timeout(&t).await;
    should_fire_close(&t).await;
    t
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
    assert!(executable.is_file());
}

// 'should handle timeout'
async fn should_handle_timeout(t: &BrowserType) {
    let result = t.launcher().timeout(0.1).launch().await;
    assert!(result.is_err());
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
    let (wait, close) = tokio::join!(
        context.expect_event(EventType::Close),
        browser.close()
    );
    close.unwrap();
    assert_eq!(wait.unwrap(), Event::Close);
}
