use super::{free_local_port, install_browser, playwright_with_driver, Which};
use playwright::api::{page, Browser, BrowserType};
use serde::Deserialize;

pub(super) async fn connect_over_cdp(which: Which) {
    let playwright = playwright_with_driver().await;
    install_browser(&playwright, which);
    let browser_type = match which {
        Which::Chromium => playwright.chromium(),
        _ => return
    };

    http(&browser_type).await;
    ws(&browser_type).await;
}

async fn http(browser_type: &BrowserType) {
    let port = free_local_port().unwrap();
    let browser = browser_type
        .launcher()
        .args(&[format!("--remote-debugging-port={}", port)])
        .launch()
        .await
        .unwrap();
    let endpoint_url = format!("http://localhost:{}", port);
    // wait needed?
    let cdp1: Browser = browser_type
        .connect_over_cdp_builder(&endpoint_url)
        .connect_over_cdp()
        .await
        .unwrap();
    let cdp2: Browser = browser_type
        .connect_over_cdp_builder(&endpoint_url)
        .connect_over_cdp()
        .await
        .unwrap();

    {
        assert_eq!(cdp1.contexts().unwrap().len(), 1);
        let page1 = cdp1.contexts().unwrap()[0].new_page().await.unwrap();
        let (a, b) = tokio::join!(
            page1.expect_event(page::EventType::DomContentLoaded),
            page1.goto_builder("https://example.com/").goto()
        );
        a.unwrap();
        b.unwrap();
        assert_eq!(cdp2.contexts().unwrap().len(), 1);
        let cdp2_pages = cdp2.contexts().unwrap()[0].pages().unwrap();
        let page2 = cdp2_pages.into_iter().next().unwrap();
        assert_eq!(page2.url().unwrap(), "https://example.com/");
    }

    cdp1.close().await.unwrap();
    cdp2.close().await.unwrap();
    browser.close().await.unwrap();
}

async fn ws(browser_type: &BrowserType) {
    let port = free_local_port().unwrap();
    let browser = browser_type
        .launcher()
        .args(&[format!("--remote-debugging-port={}", port)])
        .launch()
        .await
        .unwrap();
    let ws_endpoint = fetch_ws_endpoint(browser_type, port).await;
    {
        let cdp1 = browser_type
            .connect_over_cdp_builder(&ws_endpoint)
            .connect_over_cdp()
            .await
            .unwrap();
        assert_eq!(cdp1.contexts().unwrap().len(), 1);
        cdp1.close().await.unwrap();
    }
    {
        let cdp2 = browser_type
            .connect_over_cdp_builder(&ws_endpoint)
            .connect_over_cdp()
            .await
            .unwrap();
        assert_eq!(cdp2.contexts().unwrap().len(), 1);
        cdp2.close().await.unwrap();
    }
    browser.close().await.unwrap();
}

async fn fetch_ws_endpoint(browser_type: &BrowserType, port: u16) -> String {
    let browser = browser_type.launcher().launch().await.unwrap();
    let browser_context = browser.context_builder().build().await.unwrap();
    let page = browser_context.new_page().await.unwrap();
    let url = format!("http://localhost:{}/json/version/", port);
    let (event, goto) = tokio::join!(
        page.expect_event(page::EventType::Response),
        page.goto_builder(&url).goto()
    );
    goto.unwrap();
    let response = match event {
        Ok(page::Event::Response(res)) => res,
        _ => unreachable!()
    };
    let text = response.text().await.unwrap();
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct De<'a> {
        web_socket_debugger_url: &'a str
    }
    let De {
        web_socket_debugger_url
    } = serde_json::from_str(&text).unwrap();
    web_socket_debugger_url.into()
}
