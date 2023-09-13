use reqwest::{header::CONTENT_TYPE, StatusCode};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // test ping
    let body = format!("submit=true&value={}", "");
    let pong = client
        .post("http://localhost:3000/ping")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (pong.status(), pong.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert!(text.contains("Result: PONG"));

    // test ping with a value "Hello"
    let to_ping = "Hello";
    let body = format!("submit=true&value={}", to_ping);
    let pong = client
        .post("http://localhost:3000/ping")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (pong.status(), pong.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert!(text.contains(format!("Result: {}", to_ping).as_str()));

    // test ping without a value, expect 400
    let body = "submit=true";
    let pong = client
        .post("http://localhost:3000/ping")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = pong.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test set
    let body = "submit=true&key=foo&value=bar";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (set.status(), set.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: OK"), true);

    // test set with no key, expect 400
    let body = "submit=true&value=foo";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = set.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=&value=foo";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = set.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test set with no value, expect 400
    let body = "submit=true&key=foo";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = set.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=foo&value=";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = set.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test get
    let body = "submit=true&key=foo";
    let get = client
        .post("http://localhost:3000/get")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (get.status(), get.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: bar"), true);

    // test get with no key, expect 400
    let body = "submit=true";
    let get = client
        .post("http://localhost:3000/get")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = get.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=";
    let get = client
        .post("http://localhost:3000/get")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = get.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test del
    let body = "submit=true&key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (del.status(), del.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: 1"), true);

    let get = client
        .post("http://localhost:3000/get")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (get.status(), get.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: (nil)"), true);

    // test del twice
    let body = "submit=true&key=foo";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (del.status(), del.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: 0"), true);

    // test del with no key, expect 400
    let body = "submit=true";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = del.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = del.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=";
    let del = client
        .post("http://localhost:3000/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = del.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test set with expire time
    let body = "submit=true&key=foo&value=bar&expire=1";
    let set = client
        .post("http://localhost:3000/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let (status, text) = (set.status(), set.text().await.unwrap());
    assert_eq!(status, StatusCode::OK);
    assert_eq!(text.contains("Result: OK"), true);

    let thread = tokio::spawn(async move {
        let client = reqwest::Client::new();
        let body = "submit=true&key=foo";
        let get = client
            .post("http://localhost:3000/get")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        let (status, text) = (get.status(), get.text().await.unwrap());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(text.contains("Result: bar"), true);

        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

        let get = client
            .post("http://localhost:3000/get")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        let (status, text) = (get.status(), get.text().await.unwrap());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(text.contains("Result: (nil)"), true);
    }).await;
    assert!(thread.is_ok());

    // test subscribe and publish
    // spawn 2 subscribe threads
    let thread1 = tokio::spawn(async move {
        let client = reqwest::Client::new();
        let body = "submit=true&key=foo";
        let sub = client
            .post("http://localhost:3000/subscribe")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        let (status, text) = (sub.status(), sub.text().await.unwrap());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(text.contains("Result: ThisIsAMessage"), true);
    });

    let thread2 = tokio::spawn(async move {
        let client = reqwest::Client::new();
        let body = "submit=true&key=foo";
        let sub = client
            .post("http://localhost:3000/subscribe")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        let (status, text) = (sub.status(), sub.text().await.unwrap());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(text.contains("Result: ThisIsAMessage"), true);
    });

    let thread3 = tokio::spawn(async move {
        let client = reqwest::Client::new();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let body = "submit=true&key=foo&value=ThisIsAMessage";
        let publish = client
            .post("http://localhost:3000/publish")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        let (status, text) = (publish.status(), publish.text().await.unwrap());
        assert_eq!(status, StatusCode::OK);
        assert_eq!(text.contains("Result: 2"), true);
    });

    let joined = tokio::join!(thread1, thread2, thread3);
    assert!(joined.0.is_ok() && joined.1.is_ok() && joined.2.is_ok());

    // test subscribe with no key, expect 400
    let body = "submit=true";
    let sub = client
        .post("http://localhost:3000/subscribe")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = sub.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=";
    let sub = client
        .post("http://localhost:3000/subscribe")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = sub.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test publish with no key(channel), expect 400
    let body = "submit=true&value=ThisIsAMessage";
    let publish = client
        .post("http://localhost:3000/publish")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = publish.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=&value=ThisIsAMessage";
    let publish = client
        .post("http://localhost:3000/publish")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = publish.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // test publish with no value(message), expect 400
    let body = "submit=true&key=foo";
    let publish = client
        .post("http://localhost:3000/publish")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = publish.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let body = "submit=true&key=foo&value=";
    let publish = client
        .post("http://localhost:3000/publish")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let status = publish.status();
    assert_eq!(status, StatusCode::BAD_REQUEST);

    println!("All tests passed!")
}