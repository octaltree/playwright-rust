use playwright::Playwright;

playwright::runtime_test!(hello, {
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

    // Wait until navigated
    page.click_builder(r#"a[title="playwright::api mod"]"#)
        .click()
        .await?;
    assert_eq!(
        page.url().unwrap(),
        "https://docs.rs/playwright/0.0.5/playwright/api/index.html"
    );

    // Waiting load explicitly is unnecessary.
    // [many functions wait contents automaticaly](https://playwright.dev/docs/actionability/).
    page.expect_event(playwright::api::page::EventType::Load)
        .await?;

    Ok(())
}
