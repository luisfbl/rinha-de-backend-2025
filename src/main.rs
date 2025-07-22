use crate::server::Server;

mod handlers;
mod server;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new();
    server.run().await?;

    Ok(())
}
