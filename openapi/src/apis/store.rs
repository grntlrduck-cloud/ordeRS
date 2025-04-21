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
pub enum DeleteOrderResponse {
    /// Successful operation
    Status200_SuccessfulOperation,
    /// Invalid ID supplied
    Status400_InvalidIDSupplied,
    /// Order not found
    Status404_OrderNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetInventoryResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Inventory),
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetOrderByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Order),
    /// Invalid ID supplied
    Status400_InvalidIDSupplied,
    /// Order not found
    Status404_OrderNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlaceOrderResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Order),
    /// Invalid input
    Status400_InvalidInput,
    /// Validation exception
    Status422_ValidationException,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateOrderResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Order),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Order not found
    Status404_OrderNotFound,
    /// Validation exception
    Status422_ValidationException,
    /// Server error
    Status500_ServerError,
}

/// Store
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Store<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Delete purchase order by Id.
    ///
    /// DeleteOrder - DELETE /api/v1/store/orders/{orderId}
    async fn delete_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteOrderPathParams,
    ) -> Result<DeleteOrderResponse, E>;

    /// Returns book inventories by status.
    ///
    /// GetInventory - GET /api/v1/store/inventory
    async fn get_inventory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetInventoryResponse, E>;

    /// Find purchase order by Id..
    ///
    /// GetOrderById - GET /api/v1/store/orders/{orderId}
    async fn get_order_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetOrderByIdPathParams,
    ) -> Result<GetOrderByIdResponse, E>;

    /// Place an order for a book.
    ///
    /// PlaceOrder - POST /api/v1/store/orders
    async fn place_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewOrder,
    ) -> Result<PlaceOrderResponse, E>;

    /// Update an existing book.
    ///
    /// UpdateOrder - PATCH /api/v1/store/orders/{orderId}
    async fn update_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateOrderPathParams,
        body: &models::OrderProperties,
    ) -> Result<UpdateOrderResponse, E>;
}
