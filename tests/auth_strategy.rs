use std::sync::Arc;

use drupal_kit::http_client::HttpClient;
use drupal_kit::{auth::BasicAuthStrategy, Drupalkit};
use http::Method;

#[tokio::test]
async fn test_basic_auth() {
    let mut server = mockito::Server::new_async().await;

    let username = "mustermann1212";
    let password = "abcdef12!";

    let mock = server
        .mock("GET", "/some-path")
        .with_status(200)
        .with_body("world")
        .match_header("Authorization", "Basic bXVzdGVybWFubjEyMTI6YWJjZGVmMTIh")
        .create_async()
        .await;

    let url = server.url();

    #[cfg(not(feature = "consumer"))]
    let mut client = Drupalkit::new(&url);

    #[cfg(feature = "consumer")]
    let mut client = Drupalkit::new(&url, None);

    let auth_strategy = BasicAuthStrategy::new(username, Some(password));
    client.set_auth_strategy(auth_strategy);

    let res = client
        .request(Method::GET, "/some-path", "", vec![])
        .await
        .expect("request must not fail");

    mock.assert_async().await;

    assert!(res.status().is_success());

    let text = res.text().await.expect("must get body");
    assert_eq!("world", text);
}
