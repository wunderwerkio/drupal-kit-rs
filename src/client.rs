use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    auth::AuthStrategy,
    http_client::{HttpClient, HttpRequestOption},
};
use reqwest::Client;

#[cfg(feature = "consumer")]
const CONSUMER_HEADER_NAME: &str = "X-Consumer-ID";

#[derive(Clone)]
pub struct Drupalkit {
    pub(crate) http_client: Client,

    pub(crate) base_url: String,
    #[cfg(feature = "consumer")]
    pub(crate) client_id: Option<String>,

    pub(crate) auth_strategy: Option<Arc<Mutex<dyn AuthStrategy>>>,
}

impl Drupalkit {
    pub fn new(base_url: &str, #[cfg(feature = "consumer")] client_id: Option<&str>) -> Self {
        #[cfg(feature = "consumer")]
        let client_id = client_id.map(|client_id| client_id.to_owned());

        Self {
            http_client: reqwest::Client::new(),

            base_url: base_url.to_owned(),
            #[cfg(feature = "consumer")]
            client_id,

            auth_strategy: None,
        }
    }
}

impl HttpClient for Drupalkit {
    fn get_http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    async fn before_request(
        &self,
        req_builder: reqwest::RequestBuilder,
        path: &str,
        options: Vec<HttpRequestOption>,
    ) -> Result<reqwest::RequestBuilder, crate::http_client::ClientError> {
        // Add the X-Consumer-ID header with the client id to each request.
        #[cfg(feature = "consumer")]
        let req_builder = match &self.client_id {
            Some(client_id) => req_builder.header(CONSUMER_HEADER_NAME, client_id),
            None => req_builder,
        };

        // Do nothing if request is anonymous.
        for option in &options {
            if let HttpRequestOption::Anonymous = option {
                return Ok(req_builder);
            }
        }

        // Set auth-info from strategy if set.
        let auth_strategy = self.auth_strategy.clone();

        let req_builder = match auth_strategy {
            Some(auth_strategy) => {
                let mut rw_auth_strategy = auth_strategy.lock().await;
                rw_auth_strategy
                    .set_auth_info(req_builder, path, options, self)
                    .await?
            }
            None => req_builder,
        };

        Ok(req_builder)
    }

    async fn after_request(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response, crate::http_client::ClientError> {
        Ok(response)
    }
}
