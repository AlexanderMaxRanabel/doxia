pub async fn server_runtime() -> anyhow::Result<()> {
    Ok(())
}

async fn handler() -> &'static str {
    "Hello, World!"
}
