use drupal_kit::{http_client::HttpClient, Drupalkit};
use http::Method;

#[tokio::test]
async fn test_request() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
        .mock("GET", "/some-path")
        .with_status(200)
        .with_body("world")
        .create_async()
        .await;

    let url = server.url();

    #[cfg(not(feature = "consumer"))]
    let client = Drupalkit::new(&url);

    #[cfg(feature = "consumer")]
    let client = Drupalkit::new(&url, Some("_client-id_"));

    let res = client
        .request(Method::GET, "/some-path", "", vec![])
        .await
        .expect("request must not fail");

    assert!(res.status().is_success());

    let text = res.text().await.expect("must get body");
    assert_eq!("world", text);
}
