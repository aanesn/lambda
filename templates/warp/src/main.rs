use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "hello, world!");

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("listening on http://{}", addr);
    warp::serve(hello).run(addr).await;
}
