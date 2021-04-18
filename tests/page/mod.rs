use super::Which;
use futures::stream::StreamExt;
use playwright::api::{page, BrowserContext, Page};

pub async fn all(c: &BrowserContext, p1: Page, which: Which) {
    eq_context_close(c, &p1).await;
    focus_should_work(&p1).await;
    reload_should_worker(&p1).await;
}

async fn eq_context_close(c: &BrowserContext, p1: &Page) {
    let p2 = c.new_page().await.unwrap();
    assert_ne!(p1, &p2);
    assert_eq!(&p1.context(), c);
    assert_eq!(&p2.context(), c);
    ensure_close(&p2).await;
}

async fn ensure_close(page: &Page) {
    let mut rx = page.subscribe_event().unwrap();
    let receive_close = async {
        let mut received = false;
        while let Some(Ok(evt)) = rx.next().await {
            if let page::Event::Close = evt {
                received = true;
                break;
            }
        }
        received
    };
    let (received, wait_result, result) = tokio::join!(
        receive_close,
        page.expect_event(page::EventType::Close),
        page.close(None)
    );
    result.unwrap();
    assert_eq!(received, true);
    match wait_result.unwrap() {
        page::Event::Close => (),
        _ => unreachable!()
    }
}

async fn focus_should_work(page: &Page) {
    page.set_content_builder("<div id=d1 tabIndex=0></div>")
        .set_content()
        .await
        .unwrap();
    assert_eq!(
        page.eval::<String>("() => document.activeElement.nodeName")
            .await
            .unwrap(),
        "BODY"
    );
    page.focus("#d1", None).await.unwrap();
    assert_eq!(
        page.eval::<String>("(s) => document.activeElement.id")
            .await
            .unwrap(),
        "d1"
    );
}

async fn reload_should_worker(page: &Page) {
    page.evaluate::<i32, i32>("x => window._foo = x", 10)
        .await
        .unwrap();
    page.reload_builder().reload().await.unwrap();
    let x: Option<i32> = page.eval("() => window._foo").await.unwrap();
    assert_eq!(x, None);
}
