mod browser;
mod browser_context;
mod browser_type;
mod page;

#[cfg(feature = "rt-async-std")]
use async_std::{task::sleep, task::spawn};
#[cfg(feature = "rt-actix")]
use tokio::{task::spawn, time::sleep};
#[cfg(feature = "rt-tokio")]
use tokio::{task::spawn, time::sleep};

use playwright::Playwright;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    Webkit,
    Firefox,
    Chromium
}

playwright::runtime_test!(chromium, all(Which::Chromium).await);

playwright::runtime_test!(firefox, all(Which::Firefox).await);

// playwright::runtime_test!(webkit, all(Which::Webkit).await);

async fn all(which: Which) {
    let port = free_local_port().unwrap();
    start_test_server(port).await;
    let playwright = playwright_with_driver().await;
    let browser_type = match which {
        Which::Webkit => playwright.webkit(),
        Which::Firefox => playwright.firefox(),
        Which::Chromium => playwright.chromium()
    };
    install_browser(&playwright, which);
    let (browser, persistent) = browser_type::all(browser_type, which).await;
    let browser_context = browser::all(browser, which).await;
    assert_ne!(persistent, browser_context);
    let page = browser_context::all(&browser_context, which).await;
    page::all(&browser_context, page, port, which).await;
}

fn install_browser(p: &Playwright, which: Which) {
    match which {
        Which::Webkit => p.install_webkit(),
        Which::Firefox => p.install_firefox(),
        Which::Chromium => p.install_chromium()
    }
    .unwrap();
}

async fn playwright_with_driver() -> Playwright {
    use playwright::Driver;
    let driver = Driver::new(Driver::default_dest());
    let mut playwright = Playwright::with_driver(driver).await.unwrap();
    let _ = playwright.driver();
    playwright
}

#[cfg(any(feature = "rt-tokio", feature = "rt-actix"))]
async fn start_test_server(port: u16) {
    use warp::Filter;
    let route = warp::path("static").and(warp::fs::dir("tests/server"));
    spawn(async move {
        warp::serve(route).run(([127, 0, 0, 1], port)).await;
    });
}

#[cfg(feature = "rt-async-std")]
async fn start_test_server(port: u16) {
    use tide::Server;
    let mut app = Server::new();
    app.at("/static").serve_dir("tests/server/").unwrap();
    spawn(async move {
        app.listen(format!("127.0.0.1:{}", port)).await.unwrap();
    });
}

fn free_local_port() -> Option<u16> {
    let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
    std::net::TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

fn url_static(port: u16, path: &str) -> String {
    format!("http://localhost:{}/static{}", port, path)
}

// use playwright::{
//    api::{
//        browser_context, page, Browser, BrowserContext, BrowserType, DateTime, KeyboardModifier,
//        Page, Response
//    },
//    *
//};

// runtime_test!(awesome, {
//    let (_playwright, _browser, context, page) = init().await;
//    let _response: Option<Response> = page
//        .goto_builder("https://example.com/")
//        .goto()
//        .await
//        .unwrap();
//    let h = page.eval_handle("() => location.href").await.unwrap();
//    let s: String = page
//        .evaluate("([s]) => s + location.href", Some(vec![h]))
//        .await
//        .unwrap();
//    assert_eq!(s, "https://example.com/https://example.com/");
//    let s: DateTime = page
//        .evaluate("d => d", Some(DateTime::from(chrono::Utc::now())))
//        .await
//        .unwrap();
//    println!("{:?}", s);
//    let (next_page, _) = tokio::join!(
//        context.expect_event(browser_context::EventType::Page),
//        page.click_builder("a")
//            .modifiers(vec![KeyboardModifier::Control])
//            .click()
//    );
//    let _next_page = match next_page.unwrap() {
//        browser_context::Event::Page(p) => p,
//        _ => unreachable!()
//    };
//    ensure_timeout(&page).await;
//    //// let _ = p.main_frame().query_selector_all("a").await.unwrap();
//    //// let _ = p.main_frame().title().await.unwrap();
//    // let mut a = p.query_selector("a").await.unwrap().unwrap();
//    // let _href = a.get_attribute("href").await.unwrap();
//    // dbg!(v);
//    // p.go_back_builder().go_back().await.unwrap();
//});

// async fn ensure_timeout(page: &Page) {
//    page.set_default_timeout(500).await.unwrap();
//    match page.expect_event(page::EventType::Load).await {
//        Err(Error::Timeout) => {}
//        _ => panic!("Not expected")
//    }
//}

// async fn init() -> (Playwright, Browser, BrowserContext, Page) {
//    let pw = Playwright::initialize().await.unwrap();
//    let b = launch(&pw.chromium()).await;
//    let c = new_context(&b).await;
//    let p = c.new_page().await.unwrap();
//    (pw, b, c, p)
//}

// async fn launch(t: &BrowserType) -> Browser { t.launcher().headless(true).launch().await.unwrap() }

// async fn new_context(b: &Browser) -> BrowserContext {
//    let a = "asdf".to_string();
//    b.context_builder().user_agent(&a).build().await.unwrap()
//}
