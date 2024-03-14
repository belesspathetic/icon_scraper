use icon_scraper::crawlers::heroes::Heroes;
use icon_scraper::crawlers::items::Items;
use icon_scraper::crawlers::neutral_items::NeutralItems;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Welcome to icon_scraper!\nStarting items scrape...");
    let obj = Items::new().await?;
    obj.download_zip().await?;

    println!("Starting neutral items scrape...");
    let obj = NeutralItems::new().await?;
    obj.download_zip().await?;

    println!("Starting portrets scrape...");
    let obj = Heroes::new().await?;
    obj.download_portrets().await?;

    println!("Starting abilities scrape...");
    let obj = Heroes::new().await?;
    obj.download_abilities().await?;

    Ok(())
}
