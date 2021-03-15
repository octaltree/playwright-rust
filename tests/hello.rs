use playwright::Playwright;

playwright::runtime_test!(hello, {
    main().await.unwrap();
});

async fn main() -> Result<(), playwright::Error> {
    env_logger::builder().is_test(true).try_init().ok();
    let mut playwright = Playwright::initialize().await?; // if drop all resources are disposed
    playwright.prepare()?; // install browsers
    let mut chromium = playwright.chromium();
    let mut browser = chromium.launcher().headless(true).launch().await?;
    let mut context = browser.context_builder().build().await?;
    let mut page = context.new_page().await?;
    page.goto_builder("https://example.com/").goto().await?;

    // Exec in browser and Deserialize with serde
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    Ok(())
}
