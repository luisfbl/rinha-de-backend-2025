use crate::server::Server;

mod handlers;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new();
    server.run().await?;

    Ok(())
}
