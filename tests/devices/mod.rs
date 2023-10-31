use super::Which;
use playwright::{
    api::{page, Page, Viewport},
    Playwright
};

pub async fn all(playwright: &Playwright, port: u16, _which: Which) {
    let _devices = playwright.devices();
    let device = playwright.device("iPhone 11 Pro").unwrap();
    dbg!(&device);
    let chromium = playwright.chromium();
    let browser = chromium.launcher().launch().await.unwrap();
    let ctx = browser
        .context_builder()
        .set_device(&device)
        .build()
        .await
        .unwrap();
    let page = ctx.new_page().await.unwrap();
    check_user_agent(&page, port).await;
    check_size(&page).await;
    assert!((device_pixel_ratio(&page).await - 3.0).abs() < f64::EPSILON);
    assert!(has_touch(&page).await);
    // TODO: is_mobile
    let tmp_dir = tempdir::TempDir::new("playwright-rust").unwrap();
    dbg!(&tmp_dir);
    chromium
        .persistent_context_launcher(tmp_dir.path())
        .set_device(&device)
        .launch()
        .await
        .unwrap();
}

async fn check_size(page: &Page) {
    page.set_content_builder(
        r#"<meta name="viewport" content="width=device-width, user-scalable=no" />"#
    )
    .set_content()
    .await
    .unwrap();
    let screen: Viewport = page
        .eval("() => ({width: window.screen.width, height: window.screen.height})")
        .await
        .unwrap();
    let viewport: Viewport = page
        .eval("() => ({width: window.innerWidth, height: window.innerHeight})")
        .await
        .unwrap();
    assert_eq!(
        screen,
        Viewport {
            width: 375,
            height: 812
        }
    );
    assert_eq!(
        viewport,
        Viewport {
            width: 375,
            height: 635
        }
    );
}

async fn device_pixel_ratio(page: &Page) -> f64 {
    page.eval("window.devicePixelRatio").await.unwrap()
}

async fn has_touch(page: &Page) -> bool {
    page.eval("() => 'ontouchstart' in window").await.unwrap()
}

async fn check_user_agent(page: &Page, port: u16) {
    let user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.4 Mobile/15E148 Safari/604.1";
    assert_eq!(
        page.eval::<String>("() => navigator.userAgent")
            .await
            .unwrap(),
        user_agent
    );
    let url = super::url_static(port, "/empty.html");
    let (result, _) = tokio::join!(
        page.expect_event(page::EventType::Request),
        page.goto_builder(&url).goto()
    );
    let request = match result.unwrap() {
        page::Event::Request(request) => request,
        _ => unreachable!()
    };
    dbg!(&request.headers().unwrap());
    assert_eq!(request.headers().unwrap()["user-agent"], user_agent);
}
