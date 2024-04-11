use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::{
    auth::{AuthStrategy, AuthStrategyError, AuthStrategyResult},
    http_client::HttpRequestOption,
    Drupalkit,
};

use super::{AccessToken, SimpleOauthGrant};

pub struct ClientCredentialsAuthStrategy {
    access_token: Option<AccessToken>,

    client_id: String,
    client_secret: String,
    scopes: Vec<String>,
}

impl ClientCredentialsAuthStrategy {
    pub fn new(client_id: &str, client_secret: &str, scopes: Vec<String>) -> Self {
        Self {
            access_token: None,
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            scopes,
        }
    }
}

#[async_trait]
impl AuthStrategy for ClientCredentialsAuthStrategy {
    async fn set_auth_info(
        &mut self,
        req_builder: RequestBuilder,
        _path: &str,
        _options: Vec<HttpRequestOption>,
        drupalkit: &Drupalkit,
    ) -> AuthStrategyResult {
        // Check if cached access token exists and is still valid.
        if let Some(access_token) = &self.access_token {
            if !access_token.is_expired() {
                return Ok(req_builder.bearer_auth(access_token.value.clone()));
            }
        }

        // Request token.
        let grant = SimpleOauthGrant::ClientCredentials {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            scopes: self.scopes.clone(),
        };

        match drupalkit.request_token(grant).await {
            Ok(res) => {
                self.access_token = Some(res.clone().into());

                Ok(req_builder.bearer_auth(res.access_token))
            }
            Err(err) => Err(AuthStrategyError::new(err)),
        }
    }
}
