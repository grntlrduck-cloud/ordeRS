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
pub enum AddAuthorResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Author),
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
pub enum DeleteAuthorResponse {
    /// Successfully deleted
    Status200_SuccessfullyDeleted,
    /// Invalid authorId value
    Status400_InvalidAuthorIdValue,
    /// Author not found
    Status404_AuthorNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAuthorByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Author),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Author not found
    Status404_AuthorNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateAuthorResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Author),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Author not found
    Status404_AuthorNotFound,
    /// Validation exception
    Status422_ValidationException,
    /// Server error
    Status500_ServerError,
}

/// Author
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Author<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Add a new author to the store.
    ///
    /// AddAuthor - POST /api/v1/authors
    async fn add_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewAuthor,
    ) -> Result<AddAuthorResponse, E>;

    /// Deletes a author.
    ///
    /// DeleteAuthor - DELETE /api/v1/authors/{authorId}
    async fn delete_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteAuthorPathParams,
    ) -> Result<DeleteAuthorResponse, E>;

    /// Finds book by Id.
    ///
    /// GetAuthorById - GET /api/v1/authors/{authorId}
    async fn get_author_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetAuthorByIdPathParams,
    ) -> Result<GetAuthorByIdResponse, E>;

    /// Update an existing author.
    ///
    /// UpdateAuthor - PATCH /api/v1/authors/{authorId}
    async fn update_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateAuthorPathParams,
        body: &models::AuthorProperties,
    ) -> Result<UpdateAuthorResponse, E>;
}
