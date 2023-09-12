#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
// use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use simple_mini_redis::S;

#[volo::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(fmt::layer())
    //     .init();

    let addr: SocketAddr = "[::]:6379".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volo::example::ItemServiceServer::new(S::new())
        .run(addr)
        .await
        .unwrap();
}
