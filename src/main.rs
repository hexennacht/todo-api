pub mod application;
pub mod client;
pub mod domain;
pub mod infrastructure;
pub mod server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    server::server::start().await?;

    Ok(())
}
