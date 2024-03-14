use std::{
    env::current_dir,
    fs::{create_dir_all, File},
    io::copy,
};

use crate::{
    browser::{close, start},
    utils::{async_wait_n_milisec, async_wait_n_sec},
};
use thirtyfour::prelude::*;

const HEROES_URL: &str = "https://dota2.fandom.com/wiki/Heroes";

pub struct Heroes {
    pub heroes: Vec<String>,
    pub urls: Vec<String>,
}

impl Heroes {
    pub async fn new() -> anyhow::Result<Self> {
        let (heroes, urls) = Self::scrape_heroes().await?;

        Ok(Self {
            heroes: heroes,
            urls: urls,
        })
    }

    async fn scrape_heroes() -> anyhow::Result<(Vec<String>, Vec<String>)> {
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
                        println!("Adding: {}", atr);
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
        close(browser).await?;

        Ok((names, urls))
    }

    pub async fn download_portrets(self) -> anyhow::Result<()> {
        let mut srcs = Vec::new();
        let browser = start().await?;
        let dr = &browser.driver;

        for url in self.urls {
            dr.goto(url).await?;
            async_wait_n_sec(5).await?;
            let tbody = dr.find(By::Tag("tbody")).await?;
            let tr = tbody.find(By::Tag("tr")).await?;
            let a = tr.find(By::Tag("a")).await?;

            let img = a.find(By::Tag("img")).await?;

            let src = img.attr("src").await?;

            match src {
                Some(attr) => srcs.push(attr),
                None => continue,
            }
        }

        let cl = reqwest::Client::new();

        for (name, source) in self.heroes.iter().zip(srcs.iter()) {
            let pwd = current_dir()?;
            let path = pwd.join("img/dota2/heroes/portrets");
            create_dir_all(&path)?;
            let path_str = path.to_str().unwrap();

            let name = format!("{}/{}.png", path_str, name);
            let response = cl.get(source).send().await?;
            let mut dest = File::create(&name)?;
            copy(&mut response.bytes().await?.as_ref(), &mut dest)?;
        }

        close(browser).await?;
        Ok(())
    }

    pub async fn download_abilities(&self) -> anyhow::Result<()> {
        let browser = start().await?;
        let dr = &browser.driver;
        let pwd = current_dir()?;
        let cl = reqwest::Client::new();

        let path = pwd.join("img/dota2/heroes/abilities");
        let path_str = path.to_string_lossy().to_string();
        create_dir_all(&path)?;

        for url in &self.urls {
            dr.goto(url).await?;
            let locate_heads = dr.find_all(By::ClassName("ability-head")).await?;
            for location in locate_heads {
                location.scroll_into_view().await?;
                async_wait_n_milisec(1500).await?
            }
            let icos = dr.find_all(By::ClassName("ability-head")).await?;
            for i in icos {
                let a = i.find(By::Tag("a")).await?;
                let img = a.find(By::Tag("img")).await?;
                let alt = img.attr("alt").await?.unwrap().replace(" icon", "");
                let name = format!("{}/{}.png", path_str, alt);
                let src = img.attr("src").await?.unwrap();

                let response = cl.get(src).send().await?;
                let mut dest = File::create(&name)?;
                copy(&mut response.bytes().await?.as_ref(), &mut dest)?;
                println!("Adding: {:?}", alt);
            }
        }
        close(browser).await?;
        Ok(())
    }
}
