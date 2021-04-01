use playwright::Playwright;

playwright::runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    // main().await.unwrap();
    foo().await.unwrap();
});

async fn main() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://example.com/").goto().await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    assert_eq!(page.url().unwrap(), "https://www.iana.org/domains/reserved");
    Ok(())
}

async fn foo() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://docs.rs/playwright/0.0.5/playwright/")
        .goto()
        .await?;

    page.click_builder("a.mod").click().await?;
    assert_eq!(
        page.url().unwrap(),
        "https://docs.rs/playwright/0.0.5/playwright/api/index.html"
    );
    log::trace!("Drop connection");
    Ok(())
}
