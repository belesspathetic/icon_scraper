pub async fn async_wait_n_sec(n: u64) -> anyhow::Result<()> {
    let delay = tokio::time::Duration::from_secs(n);
    tokio::time::sleep(delay).await;

    Ok(())
}

pub async fn async_wait_n_milisec(n: u64) -> anyhow::Result<()> {
    let delay = tokio::time::Duration::from_millis(n);
    tokio::time::sleep(delay).await;

    Ok(())
}
