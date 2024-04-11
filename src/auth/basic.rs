use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::{http_client::HttpRequestOption, Drupalkit};

use super::{strategy::AuthStrategyResult, AuthStrategy};

pub struct BasicAuthStrategy {
    username: String,
    password: Option<String>,
}

impl BasicAuthStrategy {
    pub fn new(username: &str, password: Option<&str>) -> Self {
        Self {
            username: username.to_string(),
            password: password.map(|password| password.to_string()),
        }
    }
}

#[async_trait]
impl AuthStrategy for BasicAuthStrategy {
    async fn set_auth_info(
        &mut self,
        req_builder: RequestBuilder,
        _path: &str,
        _options: Vec<HttpRequestOption>,
        _drupalkit: &Drupalkit,
    ) -> AuthStrategyResult {
        let password = self.password.clone();

        Ok(req_builder.basic_auth(&self.username, password))
    }
}
