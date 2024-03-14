use std::{
    env::current_dir,
    fs::{create_dir_all, File},
    io::copy,
};

use thirtyfour::prelude::*;

use crate::{
    browser::{close, start},
    utils::{async_wait_n_milisec, async_wait_n_sec},
};

const NEUTRAL_ITEMS_URL: &str = "https://dota2.fandom.com/wiki/Neutral_Items";

pub struct NeutralItems {
    pub names: Vec<String>,
    pub sources: Vec<String>,
}

impl NeutralItems {
    pub async fn new() -> anyhow::Result<Self> {
        let (names, sources) = Self::scrape_items().await?;

        Ok(Self {
            names: names,
            sources: sources,
        })
    }

    async fn scrape_items() -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let browser = start().await?;
        let dr = &browser.driver;

        dr.goto(NEUTRAL_ITEMS_URL).await?;

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
            async_wait_n_milisec(500).await?;
            a.scroll_into_view().await?;
            async_wait_n_milisec(500).await?;
            all_a.push(a);
        }

        for a in all_a {
            let img = a.find(By::Tag("img")).await?;
            let attr = a.attr("title").await?.unwrap();
            all_names.push(attr);
            all_images.push(img);
        }

        for img in all_images {
            let source = img.attr("src").await?;
            all_sources.push(source.unwrap());
        }

        close(browser).await?;

        Ok((all_names, all_sources))
    }

    pub async fn download_zip(self) -> anyhow::Result<()> {
        let cl = reqwest::Client::new();

        for (name, source) in self.names.iter().zip(self.sources.iter()) {
            let pwd = current_dir()?;
            let path = pwd.join("img/dota2/neutral_items");
            create_dir_all(&path)?;
            let path_str = path.to_str().unwrap();
            println!("Adding: {}", name);
            let name = format!("{}/{}.png", path_str, name);
            let response = cl.get(source).send().await?;
            let mut dest = File::create(&name)?;
            copy(&mut response.bytes().await?.as_ref(), &mut dest)?;
        }

        Ok(())
    }
}
