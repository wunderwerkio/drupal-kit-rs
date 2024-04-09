use drupal_kit::{Drupalkit, HttpClient};
use http::Method;

#[cfg(feature = "simple-oauth")]
use drupal_kit::SimpleOauthGrant;

#[tokio::test]
#[cfg(not(feature = "consumer"))]
async fn test_request() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
        .mock("GET", "/some-path")
        .with_status(200)
        .with_body("world")
        .create_async()
        .await;

    let url = server.url();

    let client = Drupalkit::new(&url);

    let res = client
        .request(Method::GET, "/some-path", "", vec![])
        .await
        .expect("request must not fail");

    assert!(res.status().is_success());

    let text = res.text().await.expect("must get body");
    assert_eq!("world", text);
}

#[tokio::test]
#[cfg(feature = "consumer")]
async fn test_request_consumer() {
    let mut server = mockito::Server::new_async().await;
    let consumer_id = "_consumer-id_";

    let mock = server
        .mock("GET", "/some-path")
        .with_status(200)
        .with_body("world")
        .match_header("X-Consumer-ID", consumer_id)
        .create_async()
        .await;

    let url = server.url();

    let client = Drupalkit::new(&url, &consumer_id);

    let res = client
        .request(Method::GET, "/some-path", "", vec![])
        .await
        .expect("request must not fail");

    mock.assert();

    let text = res.text().await.expect("must get body");
    assert_eq!("world", text);
}

#[tokio::test]
#[cfg(feature = "simple-oauth")]
async fn test_request_simple_oauth() {
    let mut server = mockito::Server::new_async().await;
    let consumer_id = "_consumer-id_";

    let client_id = "_client_id_";
    let client_secret = "_client_secret_";
    let scopes = vec!["some-scope"];

    let mock = server
        .mock("GET", "/some-path")
        .with_status(200)
        .with_body("world")
        .match_header("X-Consumer-ID", consumer_id)
        .match_header("Authorization", "Bearer _access-token-value_")
        .create_async()
        .await;

    server.mock("POST", "/oauth/token")
        .with_status(200)
        .with_body(r#"{"token_type": "bearer", "expires_in": 3000, "access_token": "_access-token-value_"}"#)
        .create_async()
        .await;

    let url = server.url();

    let mut client = Drupalkit::new(&url, &consumer_id);

    let res = client
        .request_token(SimpleOauthGrant::ClientCredentials {
            client_id,
            client_secret,
            scopes,
        })
        .await;

    if let Err(err) = res {
        println!("Error: {:#?}", err);
    }

    let res = client
        .request(Method::GET, "/some-path", "", vec![])
        .await
        .expect("request must not fail");

    mock.assert();

    let text = res.text().await.expect("must get body");
    assert_eq!("world", text);
}
