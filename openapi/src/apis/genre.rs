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
pub enum AddGenreResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Genre),
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
pub enum DeleteGenreResponse {
    /// Successful operation
    Status200_SuccessfulOperation,
    /// Invalid genreId value
    Status400_InvalidGenreIdValue,
    /// Genre not found
    Status404_GenreNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetGenreByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Genre),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Genre not found
    Status404_GenreNotFound,
    /// Server error
    Status500_ServerError,
}

/// Genre
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Genre<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Add a new genre to the store.
    ///
    /// AddGenre - POST /api/v1/genres
    async fn add_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewGenre,
    ) -> Result<AddGenreResponse, E>;

    /// Deletes a genre.
    ///
    /// DeleteGenre - DELETE /api/v1/genres/{genreId}
    async fn delete_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteGenrePathParams,
    ) -> Result<DeleteGenreResponse, E>;

    /// Finds genre by Id.
    ///
    /// GetGenreById - GET /api/v1/genres/{genreId}
    async fn get_genre_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetGenreByIdPathParams,
    ) -> Result<GetGenreByIdResponse, E>;
}
