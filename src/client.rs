use crate::HttpClient;
use reqwest::Client;

#[cfg(feature = "simple-oauth")]
use crate::simple_oauth::AccessToken;

#[cfg(feature = "consumer")]
const CONSUMER_HEADER_NAME: &str = "X-Consumer-ID";

#[derive(Clone)]
pub struct Drupalkit<'lstruct> {
    http_client: Client,

    base_url: &'lstruct str,
    #[cfg(feature = "consumer")]
    client_id: &'lstruct str,
    #[cfg(feature = "simple-oauth")]
    pub(crate) access_token: Option<AccessToken>,
}

impl<'lstruct> Drupalkit<'lstruct> {
    pub fn new(
        base_url: &'lstruct str,
        #[cfg(feature = "consumer")] client_id: &'lstruct str,
    ) -> Self {
        let builder = reqwest::Client::builder();

        let http_client = builder.danger_accept_invalid_certs(true).build().unwrap();

        Self {
            http_client,

            base_url,
            #[cfg(feature = "consumer")]
            client_id,
            #[cfg(feature = "simple-oauth")]
            access_token: None,
        }
    }
}

impl HttpClient for Drupalkit<'_> {
    fn get_http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn before_request(
        &self,
        req_builder: reqwest::RequestBuilder,
    ) -> impl std::future::Future<Output = Result<reqwest::RequestBuilder, crate::ClientError>> + Send
    {
        // Add the X-Consumer-ID header with the client id to each request.
        #[cfg(feature = "consumer")]
        let req_builder = { req_builder.header(CONSUMER_HEADER_NAME, self.client_id) };

        #[cfg(feature = "simple-oauth")]
        let req_builder = match &self.access_token {
            Some(access_token) => {
                if !access_token.is_expired() {
                    req_builder.bearer_auth(access_token.value.clone())
                } else {
                    req_builder
                }
            }
            None => req_builder,
        };

        async { Ok(req_builder) }
    }

    fn after_request(
        &self,
        response: reqwest::Response,
    ) -> impl std::future::Future<Output = Result<reqwest::Response, crate::ClientError>> + Send
    {
        async { Ok(response) }
    }
}
