use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteDiscountResponse {
    /// Invalid discountId value
    Status400_InvalidDiscountIdValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDiscountByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::DiscountCode>),
    /// DiscountCode not found
    Status400_DiscountCodeNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateDiscountResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::DiscountCode),
    /// Invalid ID supplied
    Status400_InvalidIDSupplied,
    /// Discount not found
    Status404_DiscountNotFound,
    /// Validation exception
    Status422_ValidationException,
}

/// Discount
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Discount {
    /// Add a new discount to the store.
    ///
    /// AddDiscount - POST /api/v1/discount
    async fn add_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NewDiscountCode,
    ) -> Result<AddDiscountResponse, ()>;

    /// Deletes a discount.
    ///
    /// DeleteDiscount - DELETE /api/v1/discount/{discountId}
    async fn delete_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteDiscountPathParams,
    ) -> Result<DeleteDiscountResponse, ()>;

    /// Finds discount by Id.
    ///
    /// GetDiscountById - GET /api/v1/discount/{discountId}
    async fn get_discount_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetDiscountByIdPathParams,
    ) -> Result<GetDiscountByIdResponse, ()>;

    /// Update an existing discount.
    ///
    /// UpdateDiscount - PATCH /api/v1/discount/{discountId}
    async fn update_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateDiscountPathParams,
        body: models::DiscountCodeProperties,
    ) -> Result<UpdateDiscountResponse, ()>;
}
