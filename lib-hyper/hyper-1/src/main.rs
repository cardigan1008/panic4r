use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
     let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|_| async {
            Ok::<_, Infallible>(Response::builder().body(Body::empty()).unwrap())
        }))
    });

    let server = Server::bind(&"127.0.0.1:0".parse().unwrap()).serve(make_service);
    let addr = server.local_addr();

    tokio::spawn(async move {
        server.await.unwrap();
    });

    let req = Request::builder()
        .method("GET")
        .header("Transfer-Encoding", "foo")
        .uri(format!("http://{}", addr))
        .body(Body::from("foobar"))
        .unwrap();

    assert!(Client::new().request(req).await.is_ok());
}