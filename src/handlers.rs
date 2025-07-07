use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};

pub async fn payments(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    todo!()
}

pub async fn summary(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    todo!()
}
