use playwright::Playwright;

playwright::runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    main().await.unwrap();
});

async fn main() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    page.goto_builder("https://docs.rs/playwright/0.0.5/playwright/")
        .goto()
        .await?;
    // Exec js on browser and Deserialize with serde
    let url: String = page.eval("() => location.href").await?;
    assert_eq!(url, "https://docs.rs/playwright/0.0.5/playwright/");

    page.click_builder(r#"a[title="playwright::api mod"]"#)
        .click()
        .await?;
    assert_eq!(
        page.url().unwrap(),
        "https://docs.rs/playwright/0.0.5/playwright/api/index.html"
    );

    // Click waits to be navigated, so waiting afterwards times out.
    page.set_default_timeout(1000).await?;
    let waiting = page.expect_event(playwright::api::page::EventType::FrameNavigated);
    match waiting.await {
        Err(playwright::Error::Timeout) => {}
        Err(e) => return Err(e),
        Ok(_) => panic!("Not expected navigation occured")
    }

    Ok(())
}
