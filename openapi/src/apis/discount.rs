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
pub enum AddDiscountResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::DiscountCode),
    /// Invalid input
    Status400_InvalidInput,
    /// Validation exception
    Status422_ValidationException,
    /// server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteDiscountResponse {
    /// Successful operation
    Status200_SuccessfulOperation,
    /// Invalid discountId value
    Status400_InvalidDiscountIdValue,
    /// Discont not found
    Status404_DiscontNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDiscountByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::DiscountCode),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// DiscountCode not found
    Status404_DiscountCodeNotFound,
    /// Server error
    Status500_ServerError,
}

/// Discount
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Discount<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Add a new discount to the store.
    ///
    /// AddDiscount - POST /api/v1/discounts
    async fn add_discount(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewDiscountCode,
    ) -> Result<AddDiscountResponse, E>;

    /// Deletes a discount.
    ///
    /// DeleteDiscount - DELETE /api/v1/discounts/{discountId}
    async fn delete_discount(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteDiscountPathParams,
    ) -> Result<DeleteDiscountResponse, E>;

    /// Finds discount by Id.
    ///
    /// GetDiscountById - GET /api/v1/discounts/{discountId}
    async fn get_discount_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetDiscountByIdPathParams,
    ) -> Result<GetDiscountByIdResponse, E>;
}
