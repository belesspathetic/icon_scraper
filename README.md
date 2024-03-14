An web-crawler app to scrape game icons.

### 🪛 Usage
You can download all currently scraped icons in Roadmap Topic (preferred), or you can scrape them yourself.

First, you need to install `chromedriver` on your system and set it to `PATH` so app can execute it proparly. Set a path to your `chromedriver.exe` in `PATH` enviroment variable.

After that, simply run the app and wait. Do not interact with the browser. It can take a while if you scrape all of icons. All output will be saved in the current directory with named subdirectories named like this: ```./img/dota2/heroes/```

### 🛣️ Roadmap
#### Dota 2
- [x] Portrets | [download](https://github.com/belesspathetic/icon_scraper/tree/main/img/dota2/heroes/portrets)
- [x] Abilities | [download](https://github.com/belesspathetic/icon_scraper/tree/main/img/dota2/heroes/abilities)
- [x] Items | [download](https://github.com/belesspathetic/icon_scraper/tree/main/img/dota2/items)
- [x] Neutral Items  | [download](https://github.com/belesspathetic/icon_scraper/tree/main/img/dota2/neutral_items)

#### Escape From Tarkov
- [ ] - Items

### ⚙️ Tech Used
- Rust 1.76.0
  - thirtyfour 0.31.0
  - anyhow 1.0.81
  - reqwest 0.11.26
  - tokio 1.36.0 --features full

- chromedriver 122.0.6261.94

The command for running tests: `cargo test -- --test-threads=1`
