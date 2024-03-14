use icon_scraper::browser::{close, start};

const URL: &str = "https://dota2.fandom.com";

#[tokio::test]
async fn is_wiki_up() -> anyhow::Result<()> {
    let cl = reqwest::Client::new();

    let resp = cl.get(URL).send().await?;

    assert!(resp.status().is_success());

    Ok(())
}

#[tokio::test]
async fn is_driver_up() -> anyhow::Result<()> {
    let browser = start().await?;

    let status = browser.driver.status().await?;

    assert!(status.ready);

    close(browser).await?;

    Ok(())
}
