use axum::{
    middleware,
    routing::get,
    Router,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use simple_mini_redis::client_fns;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let host = "127.0.0.1";
    let port: u16 = 6379;
    client_fns::init_client(host, port);

    tracing::info!("Inited client for redis server {}:{}", host, port);

    let filter_layer = middleware::from_fn(utils::check_request);

    let app = Router::new()
        .route("/", get(utils::html_index))
        .route("/ping", get(utils::html_ping).post(utils::handler_ping))
        .route("/set", get(utils::html_set).post(utils::handler_set))
        .route("/get", get(utils::html_get).post(utils::handler_get))
        .route("/del", get(utils::html_del).post(utils::handler_del))
        .route("/subscribe", get(utils::html_subscribe).post(utils::handler_subscribe))
        .route("/publish", get(utils::html_publish).post(utils::handler_publish))
        .route_layer(filter_layer);

    let addr = "[::]:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
