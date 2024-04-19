use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::{http_client::HttpRequestOption, Drupalkit};

use super::{strategy::AuthStrategyResult, AuthStrategy};

pub struct BearerAuthStrategy {
    token: String,
}

impl BearerAuthStrategy {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

#[async_trait]
impl AuthStrategy for BearerAuthStrategy {
    async fn set_auth_info(
        &mut self,
        req_builder: RequestBuilder,
        _path: &str,
        _options: Vec<HttpRequestOption>,
        _drupalkit: &Drupalkit,
    ) -> AuthStrategyResult {
        Ok(req_builder.bearer_auth(&self.token))
    }
}
