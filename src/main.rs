pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod route;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, World!") });

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
