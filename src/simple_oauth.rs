use chrono::{DateTime, Duration, Utc};
use http::Method;
use serde::Deserialize;

use crate::{ClientError, Drupalkit, HttpClient};

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub expires_at: DateTime<Utc>,
    pub value: String,
}

impl AccessToken {
    /// Checks whether the access token is expired.
    /// This includes a one minute leeway, meaning the token
    /// is considered expired even if still valid for one minute (at max).
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        let delta = self.expires_at - now;

        delta.num_minutes() < 1
    }
}

impl From<SimpleOauthTokenResponse> for AccessToken {
    fn from(payload: SimpleOauthTokenResponse) -> Self {
        let expires_at = Utc::now() + Duration::seconds(payload.expires_in.into());

        Self {
            expires_at,
            value: payload.access_token,
        }
    }
}

pub enum SimpleOauthGrant<'a> {
    ClientCredentials {
        client_id: &'a str,
        client_secret: &'a str,
        scopes: Vec<&'a str>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleOauthTokenResponse {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl<'lstruct> Drupalkit<'lstruct> {
    pub async fn request_token(&mut self, grant: SimpleOauthGrant<'_>) -> Result<(), ClientError> {
        let body = match grant {
            SimpleOauthGrant::ClientCredentials {
                client_id,
                client_secret,
                scopes,
            } => {
                format!(
                    "client_id={}&client_secret={}&scopes={}",
                    client_id,
                    client_secret,
                    scopes.join(",")
                )
            }
        };

        let res = self
            .request_json::<SimpleOauthTokenResponse>(Method::POST, "/oauth/token", body, vec![])
            .await?;

        self.access_token = Some(res.into());

        Ok(())
    }
}
