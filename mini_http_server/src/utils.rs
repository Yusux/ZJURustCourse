use axum::{
    body::Bytes,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
    Form,
};
use serde::Deserialize;

use crate::{client_fns, htmlgen};

#[derive(Deserialize, Debug)]
pub struct ItemQuery {
    key: Option<String>,
    value: Option<String>,
    expire: Option<String>,
}

pub async fn html_index() -> Html<&'static str> {
    tracing::info!("Requesting index.html");
    Html(include_str!("../static/html/index.html"))
}

pub async fn html_ping() -> Html<&'static str> {
    tracing::info!("Requesting ping.html");
    Html(include_str!("../static/html/ping.html"))
}

pub async fn html_set() -> Html<&'static str> {
    tracing::info!("Requesting set.html");
    Html(include_str!("../static/html/set.html"))
}

pub async fn html_get() -> Html<&'static str> {
    tracing::info!("Requesting get.html");
    Html(include_str!("../static/html/get.html"))
}

pub async fn html_del() -> Html<&'static str> {
    tracing::info!("Requesting del.html");
    Html(include_str!("../static/html/del.html"))
}

pub async fn html_subscribe() -> Html<&'static str> {
    tracing::info!("Requesting subscribe.html");
    Html(include_str!("../static/html/subscribe.html"))
}

pub async fn html_publish() -> Html<&'static str> {
    tracing::info!("Requesting publish.html");
    Html(include_str!("../static/html/publish.html"))
}

pub async fn handler_ping(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let value = query.value;
    tracing::debug!("Calling ping with value: {:?}", value.as_ref());
    let result = client_fns::ping(value.unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!("Ping Result: {}", result.unwrap().unwrap());
            tracing::info!(result_mesaage);
            Html(htmlgen!("Ping", result_mesaage))
        }
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Ping", result_mesaage))
        }
    }
}

pub async fn handler_set(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key;
    let value = query.value;
    let expire = query.expire;
    match expire.is_some() && expire.as_ref().unwrap().len() > 0 {
        true => {
            tracing::debug!(
                "Calling set_ex with key: {:?}, value: {:?}, expire: {:?}",
                key.as_ref(),
                value.as_ref(),
                expire.as_ref()
            );
            let result = client_fns::set_ex(key.as_ref().unwrap().as_str(), value.as_ref().unwrap().as_str(), expire.as_ref().unwrap().as_str()).await;
            match result {
                Ok(_) => {
                    let result_mesaage = format!(
                        "Set key {} with value {} and expire {} seconds. Result: {}",
                        key.unwrap(),
                        value.unwrap(),
                        expire.unwrap(),
                        result.unwrap().unwrap()
                    );
                    tracing::info!(result_mesaage);
                    Html(htmlgen!("Set", result_mesaage))
                }
                Err(e) => {
                    let result_mesaage = e.to_string();
                    tracing::error!(result_mesaage);
                    return Html(htmlgen!("Set", result_mesaage));
                }
            }
        }
        false => {
            tracing::debug!(
                "Calling set with key: {:?}, value: {:?}",
                key.as_ref(),
                value.as_ref()
            );
            let result = client_fns::set(key.as_ref().unwrap().as_str(), value.as_ref().unwrap().as_str()).await;
            match result {
                Ok(_) => {
                    let result_mesaage = format!(
                        "Set key {} with value {}. Result: {}",
                        key.unwrap(),
                        value.unwrap(),
                        result.unwrap().unwrap()
                    );
                    tracing::info!(result_mesaage);
                    Html(htmlgen!("Set", result_mesaage))
                }
                Err(e) => {
                    let result_mesaage = e.to_string();
                    tracing::error!(result_mesaage);
                    return Html(htmlgen!("Set", result_mesaage));
                }
            }
        }
    }
}

pub async fn handler_get(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    tracing::debug!("Calling get with key: {:?}", key.as_ref());
    let result = client_fns::get(key.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!(
                "Get key {}. Result: {}",
                key.unwrap(),
                result.unwrap().unwrap()
            );
            tracing::info!(result_mesaage);
            Html(htmlgen!("Get", result_mesaage))
        }
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Get", result_mesaage))
        }
    }
}

pub async fn handler_del(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    tracing::debug!("Calling del with key: {:?}", key.as_ref());
    let result = client_fns::del(key.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!(
                "Del key {}. Result: {}",
                key.unwrap(),
                result.unwrap().unwrap()
            );
            tracing::info!(result_mesaage);
            Html(htmlgen!("Del", result_mesaage))
        }
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Del", result_mesaage))
        }
    }
}

pub async fn handler_subscribe(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    tracing::debug!("Calling subscribe with key: {:?}", key.as_ref());
    let result = client_fns::subscribe(key.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!(
                "Subscribe key {}. Result: {}",
                key.unwrap(),
                result.unwrap().unwrap()
            );
            tracing::info!(result_mesaage);
            Html(htmlgen!("Subscribe", result_mesaage))
        }
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Subscribe", result_mesaage))
        }
    }
}

pub async fn handler_publish(Form(query): Form<ItemQuery>) -> Html<String> {
    tracing::debug!("{:?}", query);
    let key = query.key.clone();
    let value = query.value.clone();
    tracing::debug!(
        "Calling publish with key: {:?}, value: {:?}",
        key.as_ref(),
        value.as_ref()
    );
    let result = client_fns::publish(key.as_ref().unwrap().as_str(), value.as_ref().unwrap().as_str()).await;
    match result {
        Ok(_) => {
            let result_mesaage = format!(
                "Publish key {} with value {}. Result: {}",
                key.unwrap(),
                value.unwrap(),
                result.unwrap().unwrap()
            );
            tracing::info!(result_mesaage);
            Html(htmlgen!("Publish", result_mesaage))
        }
        Err(e) => {
            let result_mesaage = e.to_string();
            tracing::error!(result_mesaage);
            Html(htmlgen!("Publish", result_mesaage))
        }
    }
}

// define a middleware that check the form input in the request
pub async fn check_request<B>(request: Request<B>, next: Next<B>) -> Result<impl IntoResponse, Response>
where
    B: axum::body::HttpBody + std::convert::From<axum::body::Bytes>,
    B::Error: ToString,
{
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

async fn buffer_request_body<B>(request: Request<B>) -> Result<Request<B>, Response>
where
    B: axum::body::HttpBody + std::convert::From<axum::body::Bytes>,
    B::Error: ToString,
{
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    tracing::debug!("headers: {:?}", parts.headers);

    check_request_body(parts.uri.path(), bytes.clone())?;

    Ok(Request::from_parts(parts, bytes.into()))
}

fn check_request_body(uri: &str, bytes: Bytes) -> Result<(), Response> {
    let strings: Vec<String> = std::str::from_utf8(bytes.as_ref())
        .unwrap()
        .to_string()
        .split("&")
        .map(|s| s.to_string())
        .collect();
    let mut submit = None;
    let mut key = None;
    let mut value = None;
    let mut expire = None;
    for s in strings {
        let kv: Vec<&str> = s.split("=").collect();
        match kv[0] {
            "submit" => submit = Some(kv[1].to_string()),
            "key" => key = Some(kv[1].to_string()),
            "value" => value = Some(kv[1].to_string()),
            "expire" => expire = Some(kv[1].to_string()),
            _ => {}
        }
    }
    if submit.is_none() {
        tracing::debug!("jump input check");
        return Ok(())
    }
    tracing::debug!("uri: {:?}, key: {:?}, value: {:?}, expire: {:?}", uri, key, value, expire);

    match uri {
        "/" => {
            tracing::info!("Check Request Body Success");
            Ok(())
        },
        "/ping" => {
            if value.is_none() {
                tracing::error!("Check Request Body Failed: Value Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Value Form Cannot Be Empty").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Ping Request Body Success");
            Ok(())
        },
        "/get" => {
            if key.is_none() {
                tracing::error!("Check Request Body Failed: Key Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Form Cannot Be Empty").into_response());
            }
            if key.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Key Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Cannot Be Empty").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Get Request Body Success");
            Ok(())
        },
        "/set" => {
            if key.is_none() {
                tracing::error!("Check Request Body Failed: Key Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Form Cannot Be Empty").into_response());
            }
            if value.is_none() {
                tracing::error!("Check Request Body Failed: Value Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Value Form Cannot Be Empty").into_response());
            }
            if key.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Key Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Cannot Be Empty").into_response());
            }
            if value.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Value Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Value Cannot Be Empty").into_response());
            }
            if expire.is_some() && expire.as_ref().unwrap().len() > 0 && expire.unwrap().parse::<i64>().is_err() {
                tracing::error!("Check Request Body Failed: Expire Time Must Be A Number");
                return Err((StatusCode::BAD_REQUEST, "Expire Time Must Be A Number").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Set Request Body Success");
            Ok(())
        },
        "/del" => {
            if key.is_none() {
                tracing::error!("Check Request Body Failed: Key Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Form Cannot Be Empty").into_response());
            }
            if key.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Key Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key Cannot Be Empty").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Del Request Body Success");
            Ok(())
        },
        "/subscribe" => {
            if key.is_none() {
                tracing::error!("Check Request Body Failed: Key(Channel) Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key(Channel) Form Cannot Be Empty").into_response());
            }
            if key.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Key(Channel) Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key(Channel) Cannot Be Empty").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Subscribe Request Body Success");
            Ok(())
        },
        "/publish" => {
            if key.is_none() {
                tracing::error!("Check Request Body Failed: Key(Channel) Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key(Channel) Form Cannot Be Empty").into_response());
            }
            if value.is_none() {
                tracing::error!("Check Request Body Failed: Value(Message) Form Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Value(Message) Form Cannot Be Empty").into_response());
            }
            if key.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Key(Channel) Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Key(Channel) Cannot Be Empty").into_response());
            }
            if value.as_ref().unwrap().len() == 0 {
                tracing::error!("Check Request Body Failed: Value(Message) Cannot Be Empty");
                return Err((StatusCode::BAD_REQUEST, "Value(Message) Cannot Be Empty").into_response());
            }
            if submit.unwrap() != "true" {
                tracing::error!("Check Request Body Failed: Error Submit");
                return Err((StatusCode::BAD_REQUEST, "Error Submit").into_response());
            }
            tracing::info!("Check Publish Request Body Success");
            Ok(())
        }
        _ => {
            tracing::error!("Check Request Body Failed: Key Form Cannot Be Empty");
            Err((StatusCode::BAD_REQUEST, "Bad Request").into_response())
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
            $name, $name, $result
        )
    };
}
