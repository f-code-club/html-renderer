use chromiumoxide::{
    Browser, BrowserConfig, cdp::browser_protocol::page::CaptureScreenshotFormat,
    handler::viewport::Viewport, page::ScreenshotParams,
};
use futures::StreamExt;

pub async fn render(html: &str, width: u32, height: u32) -> chromiumoxide::error::Result<Vec<u8>> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .new_headless_mode()
            .no_sandbox()
            .viewport(Some(Viewport {
                width,
                height,
                ..Default::default()
            }))
            .build()
            .expect("config for headless browser must be valid"),
    )
    .await?;
    tokio::spawn(async move {
        loop {
            let _ = handler.next().await;
        }
    });

    let page = browser.new_page("about:blank").await?;
    page.set_content(html).await?;

    let image = page
        .screenshot(
            ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(false)
                .omit_background(true)
                .build(),
        )
        .await?;

    tokio::spawn(async move {
        let _ = browser.close().await;
        let _ = browser.wait().await;
    });

    Ok(image)
}
