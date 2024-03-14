use icon_scraper::{
    browser::{close, start},
    utils::{async_wait_n_milisec, async_wait_n_sec},
};
use thirtyfour::prelude::*;

const ITEMS_URL: &str = "https://dota2.fandom.com/wiki/Items";

#[tokio::test]
async fn scrape_items() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(ITEMS_URL).await?;

    async_wait_n_sec(5).await?;
    let all_item_list = dr.find_all(By::ClassName("itemlist")).await?;
    let mut all_divs = Vec::new();
    let mut all_a = Vec::new();
    let mut all_images = Vec::new();

    let mut all_names = Vec::new();
    let mut all_sources = Vec::new();

    for list in all_item_list {
        let divs = list.find_all(By::Tag("div")).await?;
        all_divs.extend(divs)
    }

    for div in all_divs {
        let a = div.find(By::Tag("a")).await?;
        a.scroll_into_view().await?;
        async_wait_n_milisec(40).await?;
        all_a.push(a);
    }

    for a in all_a {
        let img = a.find(By::Tag("img")).await?;
        all_images.push(img)
    }

    for img in all_images {
        let source = img.attr("src").await?;
        let name = img.attr("data-image-name").await?;
        all_sources.push(source.unwrap());
        all_names.push(name.unwrap());
    }

    // println!("NAMES: {:?}", all_names);
    // println!("SOURCES: {:?}", all_sources);
    close(browser).await?;

    Ok(())
}
