use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetReadinessResponse {
    /// Successful
    Status200_Successful(models::HealthCheckResponse),
    /// Server error
    Status500_ServerError,
    /// Service unavailable
    Status503_ServiceUnavailable,
}

/// Health
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Health<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Get health status.
    ///
    /// GetReadiness - GET /api/v1/health/readiness
    async fn get_readiness(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetReadinessResponse, E>;
}
