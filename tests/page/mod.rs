use super::Which;
use futures::stream::StreamExt;
use playwright::api::{page, BrowserContext, Page};

pub async fn all(c: &BrowserContext, p1: Page, port: u16, which: Which) {
    eq_context_close(c, &p1).await;
    set_timeout(c).await;
    focus_should_work(&p1).await;
    reload_should_worker(&p1).await;
    if which != Which::Firefox {
        // XXX: go_back response is null on firefox
        navigations(&p1, port).await;
    }
    workers_should_work(&p1, port, which).await;
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

async fn navigations(page: &Page, port: u16) {
    assert_eq!(page.go_back_builder().go_back().await.unwrap(), None);
    let url1 = super::url_static(port, "/empty.html");
    let url2 = super::url_static(port, "/empty2.html");
    page.goto_builder(&url1).goto().await.unwrap();
    page.goto_builder(&url2).goto().await.unwrap();
    {
        let response = page.go_back_builder().go_back().await.unwrap().unwrap();
        assert_eq!(response.ok().unwrap(), true);
        assert_eq!(response.url().unwrap(), url1);
    }
    {
        let response = page
            .go_forward_builder()
            .go_forward()
            .await
            .unwrap()
            .unwrap();
        assert_eq!(response.ok().unwrap(), true);
        assert_eq!(response.url().unwrap(), url2);
    }
    let maybe_response = page.go_forward_builder().go_forward().await.unwrap();
    assert_eq!(maybe_response, None);
}

async fn set_timeout(c: &BrowserContext) {
    c.set_default_navigation_timeout(10000).await.unwrap();
    c.set_default_timeout(10000).await.unwrap();
}

async fn workers_should_work(page: &Page, port: u16, which: Which) {
    let url = super::url_static(port, "/worker.html");
    let js = super::url_static(port, "/worker.js");
    let empty = super::url_static(port, "/empty.html");
    let workers = || page.workers().unwrap();
    assert_eq!(workers().len(), 0);
    let (_, _) = tokio::join!(
        page.expect_event(page::EventType::Worker),
        page.goto_builder(&url).goto()
    );
    assert_eq!(workers().len(), 1);
    let w = &workers()[0];
    assert_eq!(
        w.url().unwrap(),
        match which {
            Which::Firefox => "worker.js".to_owned(),
            _ => js
        }
    );
    assert_eq!(
        w.eval::<String>("() => self.workerFunction()")
            .await
            .unwrap(),
        "worker function result"
    );
    page.goto_builder(&empty).goto().await.unwrap();
    assert_eq!(workers().len(), 0);
}
