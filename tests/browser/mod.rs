use super::Which;
use playwright::api::{Browser, BrowserContext};

pub async fn all(b: &Browser, which: Which) -> BrowserContext {
    assert_eq!(b.exists(), true);
    version_should_work(&b, which);
    contexts(&b).await
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

async fn contexts(b: &Browser) -> BrowserContext {
    let len = b.contexts().unwrap().len();
    let context = b.context_builder().build().await.unwrap();
    assert_eq!(b.contexts().unwrap().len(), len + 1);
    context.close().await.unwrap();
    assert_eq!(b.contexts().unwrap().len(), len);
    b.context_builder()
        .user_agent("asdf")
        .permissions(&["geolocation".into()])
        .build()
        .await
        .unwrap()
}
