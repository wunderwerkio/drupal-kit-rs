use drupal_kit::http::Method;
use drupal_kit::http_client::{HttpClient, HttpRequestOption};
use reqwest::Client;
use http;
use http::header;

struct TestHttpClient {
    client: Client,
    base_url: String,
}

impl TestHttpClient {
    fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

impl HttpClient for TestHttpClient {
    fn get_http_client(&self) -> &Client {
        &self.client
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

#[tokio::test]
async fn test_request_json_adds_content_type_header() {
    let mut server = mockito::Server::new_async().await;
    let base_url = server.url();

    // Create a mock that expects the Content-Type header
    let _mock = server
        .mock("GET", "/test")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    let client = TestHttpClient::new(base_url);

    // Make a request that should automatically add the Content-Type header
    let result: Result<serde_json::Value, _> =
        client.request_json(Method::GET, "/test", "", vec![]).await;

    // The request should succeed because the mock expects the Content-Type header
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_json_custom_content_type() {
    let mut server = mockito::Server::new_async().await;
    let base_url = server.url();

    // Create a mock that expects a custom Content-Type header
    let _mock = server
        .mock("GET", "/test")
        .match_header("content-type", "application/xml")
        .with_status(200)
        .with_body("{}")
        .create_async()
        .await;

    let client = TestHttpClient::new(base_url);

    // Make a request with a custom Content-Type header
    let result: Result<serde_json::Value, _> = client
        .request_json(
            Method::GET,
            "/test",
            "",
            vec![HttpRequestOption::Header(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static("application/xml"),
            )],
        )
        .await;

    // The request should succeed because the mock expects the custom Content-Type header
    assert!(result.is_ok());
}
