use super::Which;
use playwright::api::{Browser, BrowserType};

pub async fn all(t: &BrowserType, which: Which) -> Browser {
    launch_close_browser(&t).await;
    let b = launch(&t).await;
    assert!(b.exists());
    version_should_work(&b, which);
    contexts_should_work(&b).await;
    b
}

async fn launch(t: &BrowserType) -> Browser {
    t.launcher()
        .headless(false)
        .clear_headless()
        .launch()
        .await
        .unwrap()
}

async fn launch_close_browser(t: &BrowserType) {
    let (b1, b2) = tokio::join!(launch(&t), launch(&t));
    assert_ne!(b1, b2);
    b1.close().await.unwrap();
    b2.close().await.unwrap();
    assert!(!b1.exists());
}

// 'version should work'
fn version_should_work(b: &Browser, which: Which) {
    let version = b.version().unwrap();
    match which {
        Which::Chromium => {
            assert_eq!(version.split('.').count(), 4);
            for x in version.split('.') {
                x.parse::<i32>().unwrap();
            }
        }
        _ => {
            dbg!(&version);
            let mut it = version.split('.');
            it.next().unwrap().parse::<u32>().unwrap();
            let s = it.next().unwrap();
            let c: char = s.chars().next().unwrap();
            match c {
                '0'..='9' => {}
                _ => unreachable!()
            }
        }
    }
}

async fn contexts_should_work(b: &Browser) {
    let len = b.contexts().unwrap().len();
    let context = b.context_builder().build().await.unwrap();
    assert_eq!(b.contexts().unwrap().len(), len + 1);
    context.close().await.unwrap();
    context.close().await.unwrap();
    assert_eq!(b.contexts().unwrap().len(), len);
}
