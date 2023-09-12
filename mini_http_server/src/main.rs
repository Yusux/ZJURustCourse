use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
use serde::Deserialize;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use simple_mini_redis::{client_fns, S};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let host = "127.0.0.1";
    let port: u16 = 6379;
    client_fns::init_client(host, port);

    tracing::info!("Inited client for redis server {}:{}", host, port);

    let app = Router::new()
        .route("/", get(html_index))
        .route("/ping", get(html_ping).post(handler_ping))
        .route("/set", get(html_set).post(handler_set))
        .route("/get", get(html_get).post(handler_get))
        .route("/del", get(html_del).post(handler_del));

    let addr = "[::]:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct ItemQuery {
    key: Option<String>,
    value: Option<String>,
    expire: Option<String>,
}

async fn html_index() -> Html<&'static str> {
    tracing::info!("Requesting index.html");
    Html(include_str!("../static/html/index.html"))
}

async fn html_ping() -> Html<&'static str> {
    tracing::info!("Requesting ping.html");
    Html(include_str!("../static/html/ping.html"))
}

async fn html_set() -> Html<&'static str> {
    tracing::info!("Requesting set.html");
    Html(include_str!("../static/html/set.html"))
}

async fn html_get() -> Html<&'static str> {
    tracing::info!("Requesting get.html");
    Html(include_str!("../static/html/get.html"))
}

async fn html_del() -> Html<&'static str> {
    tracing::info!("Requesting del.html");
    Html(include_str!("../static/html/del.html"))
}

async fn handler_ping(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let value = query.value.clone();
    tracing::debug!("Calling ping with value: {:?}", value.as_ref());
    let result = client_fns::ping(value.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!("Ping Result: {}", result.unwrap().unwrap());
            tracing::info!(result_mesaage);
            Html(htmlgen!("Ping", result_mesaage))
        },
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Ping", result_mesaage))
        },
    }
}

async fn handler_set(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    let value = query.value.clone();
    let expired = query.expire.clone();
    match expired.as_ref().unwrap().len() > 0 {
        true => {
            tracing::debug!("Calling set_ex with key: {:?}, value: {:?}, expired: {:?}", key.as_ref(), value.as_ref(), expired.as_ref());
            let result = client_fns::set_ex(key.as_ref().unwrap().as_str(), value.as_ref().unwrap().as_str(), expired.as_ref().unwrap().as_str()).await;
            match result {
                Ok(_) => {
                    let result_mesaage = format!("Set key {} with value {} and expired {} seconds. Result: {}", key.unwrap(), value.unwrap(), expired.unwrap(), result.unwrap().unwrap());
                    tracing::info!(result_mesaage);
                    Html(htmlgen!("Set", result_mesaage))
                },
                Err(e) => {
                    let result_mesaage = e.to_string();
                    tracing::error!(result_mesaage);
                    return Html(htmlgen!("Set", result_mesaage))
                },
            }
        },
        false => {
            tracing::debug!("Calling set with key: {:?}, value: {:?}", key.as_ref(), value.as_ref());
            let result = client_fns::set(key.as_ref().unwrap().as_str(), value.as_ref().unwrap().as_str()).await;
            match result {
                Ok(_) => {
                    let result_mesaage = format!("Set key {} with value {}. Result: {}", key.unwrap(), value.unwrap(), result.unwrap().unwrap());
                    tracing::info!(result_mesaage);
                    Html(htmlgen!("Set", result_mesaage))
                },
                Err(e) => {
                    let result_mesaage = e.to_string();
                    tracing::error!(result_mesaage);
                    return Html(htmlgen!("Set", result_mesaage))
                },
            }
            
        }
    }
}

async fn handler_get(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    tracing::debug!("Calling get with key: {:?}", key.as_ref());
    let result = client_fns::get(key.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!("Get key {}. Result: {}", key.as_ref().unwrap(), result.unwrap().unwrap());
            tracing::info!(result_mesaage);
            Html(htmlgen!("Get", result_mesaage))
        },
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Get", result_mesaage))
        },
    }
}

async fn handler_del(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    tracing::debug!("Calling del with key: {:?}", key.as_ref());
    let result = client_fns::del(key.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!("Del key {}. Result: {}", key.as_ref().unwrap(), result.unwrap().unwrap());
            tracing::info!(result_mesaage);
            Html(htmlgen!("Del", result_mesaage))
        },
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Del", result_mesaage))
        },
    }
}

#[macro_export]
macro_rules! htmlgen {
    ($name:expr, $result:expr) => {
        format!(
            "<!DOCTYPE html>
            <html>
                <head>
                    <title>{} Result</title>
                </head>
                <body>
                    <h1>{} Result</h1>
                    <p>{}</p>
                    <a href=\"/\">Back</a>
                </body>
            </html>",
            $name,
            $name,
            $result
        )
    };
}