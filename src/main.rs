#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/test_api")
        .get(|_| async { Ok("Hello GET from XROAD!") })
        .post(|_| async { Ok("Hello POST from XROAD!") });
    app.listen("0.0.0.0:60000").await?;
    Ok(())
}
