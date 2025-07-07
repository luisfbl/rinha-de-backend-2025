use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use crate::handlers;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Undo hardcoded addr
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await?;

        println!("Listening on: {}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = hyper_util::rt::TokioIo::new(stream);

            tokio::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/payments") => handlers::payments(req).await,
        (&Method::GET, "/payments-summary") => handlers::summary(req).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Full::from("Not Found"))
            .unwrap()),
    }
}
