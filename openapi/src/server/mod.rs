use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A, C>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::author::Author
        + apis::book::Book
        + apis::discount::Discount
        + apis::store::Store
        + apis::ApiKeyAuthHeader<Claims = C>
        + 'static,
{
    // build our application with a route
    Router::new()
        .route("/api/v1/author", post(add_author::<I, A>))
        .route(
            "/api/v1/author/:author_id",
            delete(delete_author::<I, A>)
                .get(get_author_by_id::<I, A>)
                .patch(update_author::<I, A>),
        )
        .route("/api/v1/book", post(add_book::<I, A>))
        .route(
            "/api/v1/book/:book_id",
            delete(delete_book::<I, A>)
                .get(get_book_by_id::<I, A>)
                .patch(update_book::<I, A>),
        )
        .route(
            "/api/v1/book/findByAuthorId",
            get(get_books_by_authors::<I, A>),
        )
        .route(
            "/api/v1/book/findbyGenereId",
            get(get_books_by_generes::<I, A>),
        )
        .route(
            "/api/v1/book/findbyStatus",
            get(get_books_by_status::<I, A>),
        )
        .route("/api/v1/discount", post(add_discount::<I, A>))
        .route(
            "/api/v1/discount/:discount_id",
            delete(delete_discount::<I, A>)
                .get(get_discount_by_id::<I, A>)
                .patch(update_discount::<I, A>),
        )
        .route("/api/v1/store/inventory", get(get_inventory::<I, A>))
        .route("/api/v1/store/order", post(place_order::<I, A>))
        .route(
            "/api/v1/store/order/:order_id",
            delete(delete_order::<I, A>)
                .get(get_order_by_id::<I, A>)
                .patch(update_order::<I, A>),
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
/// AddAuthor - POST /api/v1/author
#[tracing::instrument(skip_all)]
async fn add_author<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewAuthor>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author,
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
        .add_author(method, host, cookies, body)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// DeleteAuthor - DELETE /api/v1/author/{authorId}
#[tracing::instrument(skip_all)]
async fn delete_author<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteAuthorPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author,
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
        .delete_author(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::author::DeleteAuthorResponse::Status400_InvalidAuthorIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetAuthorById - GET /api/v1/author/{authorId}
#[tracing::instrument(skip_all)]
async fn get_author_by_id<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetAuthorByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author,
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
        .get_author_by_id(method, host, cookies, path_params)
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
            apis::author::GetAuthorByIdResponse::Status400_AuthorNotFound => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// UpdateAuthor - PATCH /api/v1/author/{authorId}
#[tracing::instrument(skip_all)]
async fn update_author<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateAuthorPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::AuthorProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::author::Author,
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
        .update_author(method, host, cookies, path_params, body)
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
            apis::author::UpdateAuthorResponse::Status400_InvalidIDSupplied => {
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// AddBook - POST /api/v1/book
#[tracing::instrument(skip_all)]
async fn add_book<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewBook>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .add_book(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::AddBookResponse::Status201_SuccessfulOperation(body) => {
                let mut response = response.status(201);
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// DeleteBook - DELETE /api/v1/book/{bookId}
#[tracing::instrument(skip_all)]
async fn delete_book<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteBookPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .delete_book(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::DeleteBookResponse::Status400_InvalidBookIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetBookById - GET /api/v1/book/{bookId}
#[tracing::instrument(skip_all)]
async fn get_book_by_id<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetBookByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .get_book_by_id(method, host, cookies, path_params)
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
            apis::book::GetBookByIdResponse::Status400_BookNotFound => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetBooksByAuthors - GET /api/v1/book/findByAuthorId
#[tracing::instrument(skip_all)]
async fn get_books_by_authors<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByAuthorsQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .get_books_by_authors(method, host, cookies, query_params)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_books_by_generes_validation(
    query_params: models::GetBooksByGeneresQueryParams,
) -> std::result::Result<(models::GetBooksByGeneresQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// GetBooksByGeneres - GET /api/v1/book/findbyGenereId
#[tracing::instrument(skip_all)]
async fn get_books_by_generes<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByGeneresQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_books_by_generes_validation(query_params))
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
        .get_books_by_generes(method, host, cookies, query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::book::GetBooksByGeneresResponse::Status200_SuccessfulOperation(body) => {
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
            apis::book::GetBooksByGeneresResponse::Status400_InvalidGenreValues => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetBooksByStatus - GET /api/v1/book/findbyStatus
#[tracing::instrument(skip_all)]
async fn get_books_by_status<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::GetBooksByStatusQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .get_books_by_status(method, host, cookies, query_params)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// UpdateBook - PATCH /api/v1/book/{bookId}
#[tracing::instrument(skip_all)]
async fn update_book<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateBookPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::BookProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::book::Book,
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
        .update_book(method, host, cookies, path_params, body)
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
            apis::book::UpdateBookResponse::Status400_InvalidIDSupplied => {
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// AddDiscount - POST /api/v1/discount
#[tracing::instrument(skip_all)]
async fn add_discount<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NewDiscountCode>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount,
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
        .add_discount(method, host, cookies, body)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// DeleteDiscount - DELETE /api/v1/discount/{discountId}
#[tracing::instrument(skip_all)]
async fn delete_discount<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteDiscountPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount,
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
        .delete_discount(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::discount::DeleteDiscountResponse::Status400_InvalidDiscountIdValue => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetDiscountById - GET /api/v1/discount/{discountId}
#[tracing::instrument(skip_all)]
async fn get_discount_by_id<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetDiscountByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount,
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
        .get_discount_by_id(method, host, cookies, path_params)
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
            apis::discount::GetDiscountByIdResponse::Status400_DiscountCodeNotFound => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateDiscountBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::DiscountCodeProperties,
}

#[tracing::instrument(skip_all)]
fn update_discount_validation(
    path_params: models::UpdateDiscountPathParams,
    body: models::DiscountCodeProperties,
) -> std::result::Result<
    (
        models::UpdateDiscountPathParams,
        models::DiscountCodeProperties,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = UpdateDiscountBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateDiscount - PATCH /api/v1/discount/{discountId}
#[tracing::instrument(skip_all)]
async fn update_discount<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateDiscountPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::DiscountCodeProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::discount::Discount,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_discount_validation(path_params, body))
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
        .update_discount(method, host, cookies, path_params, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::discount::UpdateDiscountResponse::Status200_SuccessfulOperation(body) => {
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
            apis::discount::UpdateDiscountResponse::Status400_InvalidIDSupplied => {
                let mut response = response.status(400);
                response.body(Body::empty())
            }
            apis::discount::UpdateDiscountResponse::Status404_DiscountNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::discount::UpdateDiscountResponse::Status422_ValidationException => {
                let mut response = response.status(422);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// DeleteOrder - DELETE /api/v1/store/order/{orderId}
#[tracing::instrument(skip_all)]
async fn delete_order<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::DeleteOrderPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store,
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
        .delete_order(method, host, cookies, path_params)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
async fn get_inventory<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store,
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

    let result = api_impl.as_ref().get_inventory(method, host, cookies).await;

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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// GetOrderById - GET /api/v1/store/order/{orderId}
#[tracing::instrument(skip_all)]
async fn get_order_by_id<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetOrderByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store,
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
        .get_order_by_id(method, host, cookies, path_params)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
    body: Option<models::NewOrder>,
) -> std::result::Result<(Option<models::NewOrder>,), ValidationErrors> {
    if let Some(body) = &body {
        let b = PlaceOrderBodyValidator { body };
        b.validate()?;
    }

    Ok((body,))
}
/// PlaceOrder - POST /api/v1/store/order
#[tracing::instrument(skip_all)]
async fn place_order<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<Option<models::NewOrder>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store,
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
        .place_order(method, host, cookies, body)
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
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
/// UpdateOrder - PATCH /api/v1/store/order/{orderId}
#[tracing::instrument(skip_all)]
async fn update_order<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UpdateOrderPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::OrderProperties>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::store::Store,
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
        .update_order(method, host, cookies, path_params, body)
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
            apis::store::UpdateOrderResponse::Status400_InvalidIDSupplied => {
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
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
