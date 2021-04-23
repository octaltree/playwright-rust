use super::Which;
use futures::stream::StreamExt;
use playwright::api::{page, BrowserContext, Geolocation, Page, Viewport};

pub async fn all(c: &BrowserContext, port: u16, which: Which) {
    let page = c.new_page().await.unwrap();
    eq_context_close(c, &page).await;
    ensure_timeout(&page).await;
    set_timeout(&page).await;
    check_add_permissions(c, &page, port, which).await;
    front_should_work(c, &page).await;
    focus_should_work(&page).await;
    reload_should_worker(&page).await;
    viewport(&page).await;
    if which != Which::Firefox {
        // XXX: go_back response is null on firefox
        navigations(&page, port).await;
    }
    download(&page, port).await;
    workers_should_work(&page, port, which).await;
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

async fn front_should_work(c: &BrowserContext, p1: &Page) {
    let p2 = c.new_page().await.unwrap();
    p1.bring_to_front().await.unwrap();
    assert_eq!(
        p1.eval::<String>("document.visibilityState").await.unwrap(),
        "visible"
    );
    assert_eq!(
        p2.eval::<String>("document.visibilityState").await.unwrap(),
        "visible"
    );
    p2.close(None).await.unwrap();
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

async fn set_timeout(page: &Page) {
    page.set_default_navigation_timeout(10000).await.unwrap();
    page.set_default_timeout(10000).await.unwrap();
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

async fn ensure_timeout(page: &Page) {
    page.set_default_timeout(500).await.unwrap();
    match page.expect_event(page::EventType::Load).await {
        Err(playwright::Error::Timeout) => {}
        _ => panic!("Not expected")
    }
}

async fn check_add_permissions(c: &BrowserContext, page: &Page, port: u16, which: Which) {
    const PERMISSION_DENIED: i32 = 1;
    let snippet = "async () => {
        let getCurrentPositionAsync =
            () => new Promise((resolve, reject) =>
                navigator.geolocation.getCurrentPosition(resolve, reject));
        let err;
        const result = await getCurrentPositionAsync().catch(e => { err = e; });
        return [result?.coords.latitude, err?.code];
    }";
    page.goto_builder(&super::url_static(port, "/empty.html"))
        .goto()
        .await
        .unwrap();
    let geo = || async {
        page.eval::<(Option<f64>, Option<i32>)>(snippet)
            .await
            .unwrap()
    };
    assert_eq!(get_permission(page, "geolocation").await, "granted");
    c.clear_permissions().await.unwrap();
    assert_eq!(get_permission(page, "geolocation").await, "prompt");
    if which != Which::Firefox {
        // firefox shows prompt
        assert_eq!(geo().await, (None, Some(PERMISSION_DENIED)));
    }
    c.grant_permissions(&["geolocation".into()], None)
        .await
        .unwrap();
    assert_eq!(get_permission(page, "geolocation").await, "granted");
    c.set_geolocation(Some(&Geolocation {
        latitude: 59.95,
        longitude: 2.,
        accuracy: None
    }))
    .await
    .unwrap();
    let result = geo().await;
    dbg!(&result);
    assert_eq!(result.0, Some(59.95))
}

async fn get_permission(p: &Page, name: &str) -> String {
    p.evaluate(
        "name => navigator.permissions.query({name}).then(result => result.state)",
        name
    )
    .await
    .unwrap()
}

async fn viewport(p: &Page) {
    let v = Viewport {
        width: 500,
        height: 500
    };
    dbg!(p.viewport_size().unwrap());
    p.set_viewport_size(v.clone()).await.unwrap();
    assert_eq!(p.viewport_size().unwrap(), Some(v));
}

async fn download(p: &Page, port: u16) {
    p.set_content_builder(&format!(
        r#"<a href="{}">download</a>"#,
        super::url_download(port, "/worker.html")
    ))
    .set_content()
    .await
    .unwrap();
    let (d, _) = tokio::join!(
        p.expect_event(page::EventType::Download),
        p.click_builder("a").click()
    );
    let download = match d.unwrap() {
        page::Event::Download(d) => d,
        _ => unreachable!()
    };
    dbg!(download.url());
    dbg!(download.suggested_filename());
    dbg!(download.path().await.unwrap());
    assert!(!download.url().is_empty());
    assert!(!download.suggested_filename().is_empty());
    assert!(download.path().await.unwrap().is_some());
    assert_eq!(download.failure().await.unwrap(), None);
    let tmp = std::env::temp_dir().join(download.suggested_filename());
    download.save_as(tmp).await.unwrap();
    download.delete().await.unwrap();
}
