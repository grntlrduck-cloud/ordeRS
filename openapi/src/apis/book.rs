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
pub enum AddBookResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Book),
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
pub enum DeleteBookResponse {
    /// Successful operation
    Status200_SuccessfulOperation,
    /// Invalid bookId value
    Status400_InvalidBookIdValue,
    /// BookId not found
    Status404_BookIdNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetBookByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Book),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Book not found
    Status404_BookNotFound,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetBooksByAuthorsResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::Book>),
    /// Invalid author values
    Status400_InvalidAuthorValues,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetBooksByGeneresResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::Book>),
    /// Invalid genre values
    Status400_InvalidGenreValues,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetBooksByStatusResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::Book>),
    /// Invalid status value
    Status400_InvalidStatusValue,
    /// Server error
    Status500_ServerError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateBookResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Book),
    /// Invalid parameters
    Status400_InvalidParameters,
    /// Book not found
    Status404_BookNotFound,
    /// Validation exception
    Status422_ValidationException,
    /// Server error
    Status500_ServerError,
}

/// Book
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Book {
    /// Add a new book to the store.
    ///
    /// AddBook - POST /api/v1/books
    async fn add_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NewBook,
    ) -> Result<AddBookResponse, ()>;

    /// Deletes a book.
    ///
    /// DeleteBook - DELETE /api/v1/books/{bookId}
    async fn delete_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteBookPathParams,
    ) -> Result<DeleteBookResponse, ()>;

    /// Finds book by Id.
    ///
    /// GetBookById - GET /api/v1/books/{bookId}
    async fn get_book_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetBookByIdPathParams,
    ) -> Result<GetBookByIdResponse, ()>;

    /// Finds Books by AuthorId.
    ///
    /// GetBooksByAuthors - GET /api/v1/books/findByAuthorId
    async fn get_books_by_authors(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByAuthorsQueryParams,
    ) -> Result<GetBooksByAuthorsResponse, ()>;

    /// Finds Books by GenereId.
    ///
    /// GetBooksByGeneres - GET /api/v1/books/findbyGenereId
    async fn get_books_by_generes(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByGeneresQueryParams,
    ) -> Result<GetBooksByGeneresResponse, ()>;

    /// Finds Books by status.
    ///
    /// GetBooksByStatus - GET /api/v1/books/findbyStatus
    async fn get_books_by_status(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByStatusQueryParams,
    ) -> Result<GetBooksByStatusResponse, ()>;

    /// Update an existing book.
    ///
    /// UpdateBook - PATCH /api/v1/books/{bookId}
    async fn update_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateBookPathParams,
        body: models::BookProperties,
    ) -> Result<UpdateBookResponse, ()>;
}
