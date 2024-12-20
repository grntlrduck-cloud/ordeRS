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
pub enum AddAuthorResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::Author)
    ,
    /// Invalid input
    Status400_InvalidInput
    ,
    /// Validation exception
    Status422_ValidationException
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteAuthorResponse {
    /// Invalid authorId value
    Status400_InvalidAuthorIdValue
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAuthorByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation
    (Vec<models::Author>)
    ,
    /// Author not found
    Status400_AuthorNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateAuthorResponse {
    /// Successful operation
    Status200_SuccessfulOperation
    (models::Author)
    ,
    /// Invalid ID supplied
    Status400_InvalidIDSupplied
    ,
    /// Author not found
    Status404_AuthorNotFound
    ,
    /// Validation exception
    Status422_ValidationException
}


/// Author
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Author {
    /// Add a new author to the store.
    ///
    /// AddAuthor - POST /api/v1/author
    async fn add_author(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::NewAuthor,
    ) -> Result<AddAuthorResponse, ()>;

    /// Deletes a author.
    ///
    /// DeleteAuthor - DELETE /api/v1/author/{authorId}
    async fn delete_author(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::DeleteAuthorPathParams,
    ) -> Result<DeleteAuthorResponse, ()>;

    /// Finds book by Id.
    ///
    /// GetAuthorById - GET /api/v1/author/{authorId}
    async fn get_author_by_id(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetAuthorByIdPathParams,
    ) -> Result<GetAuthorByIdResponse, ()>;

    /// Update an existing author.
    ///
    /// UpdateAuthor - PATCH /api/v1/author/{authorId}
    async fn update_author(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UpdateAuthorPathParams,
            body: models::AuthorProperties,
    ) -> Result<UpdateAuthorResponse, ()>;
}
