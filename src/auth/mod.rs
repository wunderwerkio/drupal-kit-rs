mod basic;
mod strategy;

pub use basic::BasicAuthStrategy;
pub use strategy::{AuthStrategy, AuthStrategyResult, AuthStrategyError};
