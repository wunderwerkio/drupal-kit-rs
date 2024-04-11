use core::fmt;
use std::{
    error::{self, Error},
    sync::Arc,
};

use async_trait::async_trait;
use reqwest::RequestBuilder;
use tokio::sync::Mutex;

use crate::{http_client::HttpRequestOption, Drupalkit};

pub type AuthStrategyResult = Result<RequestBuilder, AuthStrategyError>;

#[derive(Debug)]
pub struct AuthStrategyError {
    source: Box<dyn Error + Send + Sync>,
}

impl AuthStrategyError {
    pub fn new(source: Box<dyn Error + Send + Sync>) -> Self {
        Self { source }
    }
}

impl fmt::Display for AuthStrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not set auth info for request: {}", self.source)
    }
}

impl error::Error for AuthStrategyError {}

#[async_trait]
pub trait AuthStrategy
where
    Self: Send + Sync,
{
    async fn set_auth_info(
        &mut self,
        req_builder: RequestBuilder,
        path: &str,
        options: Vec<HttpRequestOption>,
        drupalkit: &Drupalkit,
    ) -> AuthStrategyResult;
}

impl Drupalkit {
    pub fn set_auth_strategy<T>(&mut self, auth_strategy: T) -> &Self
    where
        T: AuthStrategy + 'static,
    {
        self.auth_strategy = Some(Arc::new(Mutex::new(auth_strategy)));

        self
    }
}
