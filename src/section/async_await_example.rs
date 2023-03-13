#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = tokio::fs::read("file.text").await?;
    Ok(())
}
