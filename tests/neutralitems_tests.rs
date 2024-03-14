use icon_scraper::{
    browser::{close, start},
    utils::async_wait_n_sec,
};
use thirtyfour::prelude::*;

const NEUTRAL_ITEMS_URL: &str = "https://dota2.fandom.com/wiki/Neutral_Items";

#[tokio::test]
async fn neutrals_items() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(NEUTRAL_ITEMS_URL).await?;

    close(browser).await?;
    Ok(())
}

#[tokio::test]
async fn get_item_list_element() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(NEUTRAL_ITEMS_URL).await?;

    let item_list_el = dr.find(By::ClassName("itemlist")).await?;

    assert!(item_list_el.is_present().await?);
    close(browser).await?;
    Ok(())
}

#[tokio::test]
async fn get_a_element() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(NEUTRAL_ITEMS_URL).await?;

    let item_list_el = dr.find(By::ClassName("itemlist")).await?;

    let a = item_list_el.find(By::Tag("a")).await?;

    assert!(a.is_present().await?);
    close(browser).await?;
    Ok(())
}

#[tokio::test]
async fn get_img_element() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(NEUTRAL_ITEMS_URL).await?;

    let item_list_el = dr.find(By::ClassName("itemlist")).await?;

    let a = item_list_el.find(By::Tag("a")).await?;

    let img = a.find(By::Tag("img")).await?;

    assert!(img.is_present().await?);
    close(browser).await?;
    Ok(())
}

#[tokio::test]
async fn get_attr() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(NEUTRAL_ITEMS_URL).await?;

    let item_list_el = dr.find(By::ClassName("itemlist")).await?;

    item_list_el
        .find(By::Tag("a"))
        .await?
        .scroll_into_view()
        .await?;

    async_wait_n_sec(2).await?;
    let a = item_list_el.find(By::Tag("a")).await?;

    let name = a.attr("title").await?.unwrap_or("X".to_string());

    let img = a.find(By::Tag("img")).await?;

    let src = img.attr("src").await?.unwrap_or("X".to_string());

    assert_eq!(name, "Arcane Ring".to_string());
    assert_eq!(src, "https://static.wikia.nocookie.net/dota2_gamepedia/images/a/ad/Arcane_Ring_icon.png/revision/latest/scale-to-width-down/60?cb=20191126192843".to_string());
    close(browser).await?;
    Ok(())
}
