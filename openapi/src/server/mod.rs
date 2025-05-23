use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A, E, C>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::author::Author<E>
        + apis::book::Book<E>
        + apis::discount::Discount<E>
        + apis::genre::Genre<E>
        + apis::health::Health<E>
        + apis::store::Store<E>
        + apis::ApiKeyAuthHeader<Claims = C>
        + Send
        + Sync
        + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // build our application with a route
    Router::new()
        .route("/api/v1/authors", post(add_author::<I, A, E>))
        .route(
            "/api/v1/authors/{author_id}",
            delete(delete_author::<I, A, E>)
                .get(get_author_by_id::<I, A, E>)
                .patch(update_author::<I, A, E>),
        )
        .route("/api/v1/books", post(add_book::<I, A, E>))
        .route(
            "/api/v1/books/findByAuthorId",
            get(get_books_by_authors::<I, A, E>),
        )
        .route(
            "/api/v1/books/findByGenreId",
            get(get_books_by_genres::<I, A, E>),
        )
        .route(
            "/api/v1/books/findByStatus",
            get(get_books_by_status::<I, A, E>),
        )
        .route(
            "/api/v1/books/{book_id}",
            delete(delete_book::<I, A, E>)
                .get(get_book_by_id::<I, A, E>)
                .patch(update_book::<I, A, E>),
        )
        .route("/api/v1/discounts", post(add_discount::<I, A, E>))
        .route(
            "/api/v1/discounts/{discount_id}",
            delete(delete_discount::<I, A, E>).get(get_discount_by_id::<I, A, E>),
        )
        .route("/api/v1/genres", post(add_genre::<I, A, E>))
        .route(
            "/api/v1/genres/{genre_id}",
            delete(delete_genre::<I, A, E>).get(get_genre_by_id::<I, A, E>),
        )
        .route("/api/v1/health/readiness", get(get_readiness::<I, A, E>))
        .route("/api/v1/store/inventory", get(get_inventory::<I, A, E>))
        .route("/api/v1/store/orders", post(place_order::<I, A, E>))
        .route(
            "/api/v1/store/orders/{order_id}",
            delete(delete_order::<I, A, E>)
                .get(get_order_by_id::<I, A, E>)
                .patch(update_order::<I, A, E>),
        )
        .with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AddAuthorBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NewAuthor,
}

#[tracing::instrument(skip_all)]
fn add_author_validation(
    body: models::NewAuthor,
) -> std::result::Result<(models::NewAuthor,), ValidationErrors> {
    let b = AddAuthorBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// AddAuthor - POST /api/v1/authors
#[tracing::instrument(skip_all)]
async fn add_author<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewAuthor>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || add_author_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .add_author(&method, &host, &cookies, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::author::AddAuthorResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::author::AddAuthorResponse::Status400_InvalidInput => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::author::AddAuthorResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::author::AddAuthorResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_author_validation(
    path_params: models::DeleteAuthorPathParams,
) -> std::result::Result<(models::DeleteAuthorPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteAuthor - DELETE /api/v1/authors/{authorId}
#[tracing::instrument(skip_all)]
async fn delete_author<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteAuthorPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_author_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_author(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::author::DeleteAuthorResponse::Status200_SuccessfullyDeleted => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::author::DeleteAuthorResponse::Status400_InvalidAuthorIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::author::DeleteAuthorResponse::Status404_AuthorNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::author::DeleteAuthorResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_author_by_id_validation(
    path_params: models::GetAuthorByIdPathParams,
) -> std::result::Result<(models::GetAuthorByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetAuthorById - GET /api/v1/authors/{authorId}
#[tracing::instrument(skip_all)]
async fn get_author_by_id<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetAuthorByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_author_by_id_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_author_by_id(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::author::GetAuthorByIdResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::author::GetAuthorByIdResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::author::GetAuthorByIdResponse::Status404_AuthorNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::author::GetAuthorByIdResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateAuthorBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::AuthorProperties,
}

#[tracing::instrument(skip_all)]
fn update_author_validation(
    path_params: models::UpdateAuthorPathParams,
    body: models::AuthorProperties,
) -> std::result::Result<(models::UpdateAuthorPathParams, models::AuthorProperties), ValidationErrors>
{
    path_params.validate()?;
    let b = UpdateAuthorBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateAuthor - PATCH /api/v1/authors/{authorId}
#[tracing::instrument(skip_all)]
async fn update_author<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateAuthorPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::AuthorProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_author_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_author(&method, &host, &cookies, &path_params, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::author::UpdateAuthorResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::author::UpdateAuthorResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::author::UpdateAuthorResponse::Status404_AuthorNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::author::UpdateAuthorResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::author::UpdateAuthorResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AddBookBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NewBook,
}

#[tracing::instrument(skip_all)]
fn add_book_validation(
    body: models::NewBook,
) -> std::result::Result<(models::NewBook,), ValidationErrors> {
    let b = AddBookBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// AddBook - POST /api/v1/books
#[tracing::instrument(skip_all)]
async fn add_book<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewBook>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || add_book_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .add_book(&method, &host, &cookies, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::AddBookResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::AddBookResponse::Status400_InvalidInput => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::AddBookResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::book::AddBookResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_book_validation(
    path_params: models::DeleteBookPathParams,
) -> std::result::Result<(models::DeleteBookPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteBook - DELETE /api/v1/books/{bookId}
#[tracing::instrument(skip_all)]
async fn delete_book<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteBookPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_book_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_book(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::DeleteBookResponse::Status200_SuccessfulOperation => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::book::DeleteBookResponse::Status400_InvalidBookIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::DeleteBookResponse::Status404_BookIdNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::book::DeleteBookResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_book_by_id_validation(
    path_params: models::GetBookByIdPathParams,
) -> std::result::Result<(models::GetBookByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetBookById - GET /api/v1/books/{bookId}
#[tracing::instrument(skip_all)]
async fn get_book_by_id<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetBookByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_book_by_id_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_book_by_id(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::GetBookByIdResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::GetBookByIdResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::GetBookByIdResponse::Status404_BookNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::book::GetBookByIdResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_books_by_authors_validation(
    query_params: models::GetBooksByAuthorsQueryParams,
) -> std::result::Result<(models::GetBooksByAuthorsQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// GetBooksByAuthors - GET /api/v1/books/findByAuthorId
#[tracing::instrument(skip_all)]
async fn get_books_by_authors<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByAuthorsQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_books_by_authors_validation(query_params))
            .await
            .unwrap();

    let Ok((query_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_books_by_authors(&method, &host, &cookies, &query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::GetBooksByAuthorsResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::GetBooksByAuthorsResponse::Status400_InvalidAuthorValues => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::GetBooksByAuthorsResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_books_by_genres_validation(
    query_params: models::GetBooksByGenresQueryParams,
) -> std::result::Result<(models::GetBooksByGenresQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// GetBooksByGenres - GET /api/v1/books/findByGenreId
#[tracing::instrument(skip_all)]
async fn get_books_by_genres<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByGenresQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_books_by_genres_validation(query_params))
            .await
            .unwrap();

    let Ok((query_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_books_by_genres(&method, &host, &cookies, &query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::GetBooksByGenresResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::GetBooksByGenresResponse::Status400_InvalidGenreValues => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::GetBooksByGenresResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_books_by_status_validation(
    query_params: models::GetBooksByStatusQueryParams,
) -> std::result::Result<(models::GetBooksByStatusQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// GetBooksByStatus - GET /api/v1/books/findByStatus
#[tracing::instrument(skip_all)]
async fn get_books_by_status<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByStatusQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_books_by_status_validation(query_params))
            .await
            .unwrap();

    let Ok((query_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_books_by_status(&method, &host, &cookies, &query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::GetBooksByStatusResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::GetBooksByStatusResponse::Status400_InvalidStatusValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::GetBooksByStatusResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateBookBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::BookProperties,
}

#[tracing::instrument(skip_all)]
fn update_book_validation(
    path_params: models::UpdateBookPathParams,
    body: models::BookProperties,
) -> std::result::Result<(models::UpdateBookPathParams, models::BookProperties), ValidationErrors> {
    path_params.validate()?;
    let b = UpdateBookBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateBook - PATCH /api/v1/books/{bookId}
#[tracing::instrument(skip_all)]
async fn update_book<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateBookPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::BookProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || update_book_validation(path_params, body))
        .await
        .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_book(&method, &host, &cookies, &path_params, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::UpdateBookResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::book::UpdateBookResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::book::UpdateBookResponse::Status404_BookNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::book::UpdateBookResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::book::UpdateBookResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AddDiscountBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NewDiscountCode,
}

#[tracing::instrument(skip_all)]
fn add_discount_validation(
    body: models::NewDiscountCode,
) -> std::result::Result<(models::NewDiscountCode,), ValidationErrors> {
    let b = AddDiscountBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// AddDiscount - POST /api/v1/discounts
#[tracing::instrument(skip_all)]
async fn add_discount<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewDiscountCode>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || add_discount_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .add_discount(&method, &host, &cookies, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::discount::AddDiscountResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::discount::AddDiscountResponse::Status400_InvalidInput => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::discount::AddDiscountResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::discount::AddDiscountResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_discount_validation(
    path_params: models::DeleteDiscountPathParams,
) -> std::result::Result<(models::DeleteDiscountPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteDiscount - DELETE /api/v1/discounts/{discountId}
#[tracing::instrument(skip_all)]
async fn delete_discount<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteDiscountPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_discount_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_discount(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::discount::DeleteDiscountResponse::Status200_SuccessfulOperation => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::discount::DeleteDiscountResponse::Status400_InvalidDiscountIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::discount::DeleteDiscountResponse::Status404_DiscountNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::discount::DeleteDiscountResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_discount_by_id_validation(
    path_params: models::GetDiscountByIdPathParams,
) -> std::result::Result<(models::GetDiscountByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetDiscountById - GET /api/v1/discounts/{discountId}
#[tracing::instrument(skip_all)]
async fn get_discount_by_id<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetDiscountByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_discount_by_id_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_discount_by_id(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::discount::GetDiscountByIdResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::discount::GetDiscountByIdResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::discount::GetDiscountByIdResponse::Status404_DiscountCodeNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::discount::GetDiscountByIdResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct AddGenreBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NewGenre,
}

#[tracing::instrument(skip_all)]
fn add_genre_validation(
    body: models::NewGenre,
) -> std::result::Result<(models::NewGenre,), ValidationErrors> {
    let b = AddGenreBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// AddGenre - POST /api/v1/genres
#[tracing::instrument(skip_all)]
async fn add_genre<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewGenre>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::genre::Genre<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || add_genre_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .add_genre(&method, &host, &cookies, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::genre::AddGenreResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::genre::AddGenreResponse::Status400_InvalidInput => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::genre::AddGenreResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::genre::AddGenreResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_genre_validation(
    path_params: models::DeleteGenrePathParams,
) -> std::result::Result<(models::DeleteGenrePathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteGenre - DELETE /api/v1/genres/{genreId}
#[tracing::instrument(skip_all)]
async fn delete_genre<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteGenrePathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::genre::Genre<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_genre_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_genre(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::genre::DeleteGenreResponse::Status200_SuccessfulOperation => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::genre::DeleteGenreResponse::Status400_InvalidGenreIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::genre::DeleteGenreResponse::Status404_GenreNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::genre::DeleteGenreResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_genre_by_id_validation(
    path_params: models::GetGenreByIdPathParams,
) -> std::result::Result<(models::GetGenreByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetGenreById - GET /api/v1/genres/{genreId}
#[tracing::instrument(skip_all)]
async fn get_genre_by_id<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetGenreByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::genre::Genre<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_genre_by_id_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_genre_by_id(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::genre::GetGenreByIdResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::genre::GetGenreByIdResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::genre::GetGenreByIdResponse::Status404_GenreNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::genre::GetGenreByIdResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_readiness_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetReadiness - GET /api/v1/health/readiness
#[tracing::instrument(skip_all)]
async fn get_readiness<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::health::Health<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_readiness_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_readiness(&method, &host, &cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::health::GetReadinessResponse::Status200_Successful(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::health::GetReadinessResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
            apis::health::GetReadinessResponse::Status503_ServiceUnavailable => {
                let mut response = response.status(503);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_order_validation(
    path_params: models::DeleteOrderPathParams,
) -> std::result::Result<(models::DeleteOrderPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteOrder - DELETE /api/v1/store/orders/{orderId}
#[tracing::instrument(skip_all)]
async fn delete_order<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteOrderPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_order_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_order(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::store::DeleteOrderResponse::Status200_SuccessfulOperation => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::store::DeleteOrderResponse::Status400_InvalidIDSupplied => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::store::DeleteOrderResponse::Status404_OrderNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::store::DeleteOrderResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_inventory_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetInventory - GET /api/v1/store/inventory
#[tracing::instrument(skip_all)]
async fn get_inventory<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_inventory_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_inventory(&method, &host, &cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::store::GetInventoryResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::store::GetInventoryResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_order_by_id_validation(
    path_params: models::GetOrderByIdPathParams,
) -> std::result::Result<(models::GetOrderByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetOrderById - GET /api/v1/store/orders/{orderId}
#[tracing::instrument(skip_all)]
async fn get_order_by_id<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetOrderByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_order_by_id_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_order_by_id(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::store::GetOrderByIdResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::store::GetOrderByIdResponse::Status400_InvalidIDSupplied => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::store::GetOrderByIdResponse::Status404_OrderNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::store::GetOrderByIdResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct PlaceOrderBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NewOrder,
}

#[tracing::instrument(skip_all)]
fn place_order_validation(
    body: models::NewOrder,
) -> std::result::Result<(models::NewOrder,), ValidationErrors> {
    let b = PlaceOrderBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// PlaceOrder - POST /api/v1/store/orders
#[tracing::instrument(skip_all)]
async fn place_order<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewOrder>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || place_order_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .place_order(&method, &host, &cookies, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::store::PlaceOrderResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::store::PlaceOrderResponse::Status400_InvalidInput => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::store::PlaceOrderResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::store::PlaceOrderResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateOrderBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::OrderProperties,
}

#[tracing::instrument(skip_all)]
fn update_order_validation(
    path_params: models::UpdateOrderPathParams,
    body: models::OrderProperties,
) -> std::result::Result<(models::UpdateOrderPathParams, models::OrderProperties), ValidationErrors>
{
    path_params.validate()?;
    let b = UpdateOrderBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateOrder - PATCH /api/v1/store/orders/{orderId}
#[tracing::instrument(skip_all)]
async fn update_order<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateOrderPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::OrderProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_order_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_order(&method, &host, &cookies, &path_params, &body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::store::UpdateOrderResponse::Status200_SuccessfulOperation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::store::UpdateOrderResponse::Status400_InvalidParameters => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::store::UpdateOrderResponse::Status404_OrderNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::store::UpdateOrderResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
            apis::store::UpdateOrderResponse::Status500_ServerError => {
                let mut response = response.status(500);
                response.body(Body::empty())
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
