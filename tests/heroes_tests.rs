use icon_scraper::{
    browser::{close, start},
    utils::async_wait_n_sec,
};
use thirtyfour::prelude::*;

const HEROES_URL: &str = "https://dota2.fandom.com/wiki/Heroes";

#[tokio::test]
async fn scrape_abilities() -> anyhow::Result<()> {
    let browser = start().await?;
    let dr = &browser.driver;
    dr.goto("https://dota2.fandom.com/wiki/Abaddon").await?;

    let ability_head = dr.find(By::ClassName("ico_active")).await?;

    let a = ability_head.find(By::Tag("a")).await?;
    a.scroll_into_view().await?;
    async_wait_n_sec(1).await?;
    let img = a.find(By::Tag("img")).await?;

    let alt = img.attr("alt").await?.unwrap();
    let src = img.attr("src").await?.unwrap();

    assert_eq!(alt, "Mist Coil icon".to_string());
    assert_eq!(src, "https://static.wikia.nocookie.net/dota2_gamepedia/images/c/ce/Mist_Coil_icon.png/revision/latest?cb=20130710221942".to_string());

    close(browser).await?;
    Ok(())
}

#[tokio::test]
async fn scrape_urls() -> anyhow::Result<()> {
    let mut urls = Vec::new();
    let mut names = Vec::new();

    let browser = start().await?;
    let dr = &browser.driver;

    dr.goto(HEROES_URL).await?;

    let trs = dr.find_all(By::Tag("tr")).await?;

    for tr in trs {
        let all_a = tr.find_all(By::Tag("a")).await?;
        for a in all_a {
            let href = a.attr("href").await?;
            let name = a.attr("title").await?;
            match name {
                Some(atr) => {
                    if atr == "Strength"
                        || atr == "Agility"
                        || atr == "Intelligence"
                        || atr == "Universal"
                    {
                        continue;
                    }
                    names.push(atr)
                }
                None => continue,
            }

            match href {
                Some(atr) => {
                    if atr == "/wiki/Strength"
                        || atr == "/wiki/Agility"
                        || atr == "/wiki/Intelligence"
                        || atr == "/wiki/Universal"
                    {
                        continue;
                    }

                    let url = format!("https://dota2.fandom.com{}", atr);
                    urls.push(url);
                }
                None => continue,
            }
        }
    }

    // println!("Names:\n{:?}", names);
    // println!("Urls:\n{:?}", urls);

    assert!(!urls.is_empty());
    assert!(!names.is_empty());
    close(browser).await?;
    Ok(())
}
