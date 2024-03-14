use crate::utils::async_wait_n_sec;
use std::process::{Child, Command, Stdio};
use thirtyfour::{prelude::*, ChromeCapabilities};

pub struct Browser {
    pub driver: WebDriver,
    pub command: Child,
}

pub async fn start() -> anyhow::Result<Browser> {
    let command = Command::new("chromedriver")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    async_wait_n_sec(3).await?;
    let mut caps = ChromeCapabilities::new();
    caps.add_chrome_arg("--ignore-certificate-errors-spki-list")?;
    caps.add_chrome_arg("--disable-logging")?;
    caps.add_chrome_arg("--disable-crash-reporter")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;
    caps.add_chrome_arg("--output=/dev/null")?;
    caps.add_chrome_arg("--disable-in-process-stack-traces")?;
    caps.add_chrome_arg("--log-level=3")?;


    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let browser = Browser {
        driver: driver,
        command: command,
    };

    Ok(browser)
}

pub async fn close(browser: Browser) -> anyhow::Result<()> {
    let driver = browser.driver;
    driver.quit().await?;

    let mut command = browser.command;
    command.kill()?;

    Ok(())
}
