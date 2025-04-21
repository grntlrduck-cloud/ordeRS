use async_trait::async_trait;
use axum::http::header::HeaderMap;
use axum_extra::extract::{CookieJar, Host};
use http::Method;
use openapi::apis::{ApiKeyAuthHeader, author, book, discount, genre, health, store};
use openapi::models;
use std::str::FromStr;
use std::sync::Arc;
use svix_ksuid::Ksuid;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber;

use crate::domain;

use super::domain_mappers::*;
use super::rest_mappers::*;

/// TODO: implement function bodies
/// here will come the implementation of the API handler
pub struct BookStoreServer {
    order_service: Arc<dyn domain::store::OrderHandler + Send + Sync>,
    book_service: Arc<dyn domain::store::BookHandler + Send + Sync>,
}

impl openapi::apis::ErrorHandler for BookStoreServer {}

pub async fn start_server(addr: &str) {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let order_service = domain::order_service::OrderService::new(String::from("TODO"));

    let book_service = domain::book_service::BookService::new(String::from("TODO"));

    // Init Axum router
    let app = openapi::server::new(Arc::new(BookStoreServer {
        order_service,
        book_service,
    }));

    // Add layers to the router
    //let app = app.layer(...);

    // Run the server with graceful shutdown
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        // TODO:: handle shut down with DB connection
        _ = ctrl_c => println!("Received ctrl + c"),
        _ = terminate => println!("Received terminate"),
    }
}

#[allow(unused_variables)]
#[async_trait]
impl health::Health for BookStoreServer {
    async fn get_readiness(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<health::GetReadinessResponse, ()> {
        Ok(health::GetReadinessResponse::Status200_Successful(
            models::HealthCheckResponse {
                status: Some(String::from("serving")),
            },
        ))
    }
}

#[allow(unused_variables)]
#[async_trait]
impl author::Author for BookStoreServer {
    async fn add_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewAuthor,
    ) -> Result<author::AddAuthorResponse, ()> {
        let domain = map_new_author_to_domain(body);
        match self.book_service.create_author(domain).await {
            Ok(result) => {
                let model = map_author_to_rest(result);
                Ok(author::AddAuthorResponse::Status200_SuccessfulOperation(
                    model,
                ))
            }
            Err(_) => Ok(author::AddAuthorResponse::Status500_ServerError),
        }
    }

    async fn delete_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteAuthorPathParams,
    ) -> Result<author::DeleteAuthorResponse, ()> {
        match Ksuid::from_str(&path_params.author_id) {
            Ok(id) => match self.book_service.delte_author_by_id(id).await {
                Ok(author) => Ok(author::DeleteAuthorResponse::Status200_SuccessfullyDeleted),
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(author::DeleteAuthorResponse::Status404_AuthorNotFound)
                }
                Err(_) => Ok(author::DeleteAuthorResponse::Status500_ServerError),
            },
            Err(_) => Ok(author::DeleteAuthorResponse::Status400_InvalidAuthorIdValue),
        }
    }

    async fn get_author_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetAuthorByIdPathParams,
    ) -> Result<author::GetAuthorByIdResponse, ()> {
        match Ksuid::from_str(&path_params.author_id) {
            Ok(id) => match self.book_service.get_author_by_id(id).await {
                Ok(author) => {
                    let model = map_author_to_rest(author);
                    Ok(author::GetAuthorByIdResponse::Status200_SuccessfulOperation(model))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(author::GetAuthorByIdResponse::Status404_AuthorNotFound)
                }
                Err(_) => Ok(author::GetAuthorByIdResponse::Status500_ServerError),
            },
            Err(_) => Ok(author::GetAuthorByIdResponse::Status400_InvalidParameters),
        }
    }

    async fn update_author(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateAuthorPathParams,
        body: &models::AuthorProperties,
    ) -> Result<author::UpdateAuthorResponse, ()> {
        match map_author_update_props_to_domain(&path_params.author_id, body) {
            Ok(props) => match self.book_service.update_author(props).await {
                Ok(author) => {
                    let model = map_author_to_rest(author);
                    Ok(author::UpdateAuthorResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(author::UpdateAuthorResponse::Status404_AuthorNotFound)
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(author::UpdateAuthorResponse::Status422_ValidationException)
                }
                Err(_) => Ok(author::UpdateAuthorResponse::Status500_ServerError),
            },
            Err(_) => Ok(author::UpdateAuthorResponse::Status400_InvalidParameters),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl book::Book for BookStoreServer {
    async fn add_book(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewBook,
    ) -> Result<book::AddBookResponse, ()> {
        match map_new_book_to_domain(body) {
            Ok(new_book) => match self.book_service.create_book(new_book).await {
                Ok(book) => {
                    let model = map_book_to_rest(book);
                    Ok(book::AddBookResponse::Status200_SuccessfulOperation(model))
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(book::AddBookResponse::Status422_ValidationException)
                }
                Err(_) => Ok(book::AddBookResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::AddBookResponse::Status400_InvalidInput),
        }
    }

    async fn delete_book(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteBookPathParams,
    ) -> Result<book::DeleteBookResponse, ()> {
        match Ksuid::from_str(&path_params.book_id) {
            Ok(id) => match self.book_service.delete_book_by_id(id).await {
                Ok(_) => Ok(book::DeleteBookResponse::Status200_SuccessfulOperation),
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(book::DeleteBookResponse::Status404_BookIdNotFound)
                }
                Err(_) => Ok(book::DeleteBookResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::DeleteBookResponse::Status400_InvalidBookIdValue),
        }
    }

    async fn get_book_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetBookByIdPathParams,
    ) -> Result<book::GetBookByIdResponse, ()> {
        match Ksuid::from_str(&path_params.book_id) {
            Ok(id) => match self.book_service.get_book_by_id(id).await {
                Ok(book) => {
                    let model = map_book_to_rest(book);
                    Ok(book::GetBookByIdResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(book::GetBookByIdResponse::Status404_BookNotFound)
                }
                Err(_) => Ok(book::GetBookByIdResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::GetBookByIdResponse::Status400_InvalidParameters),
        }
    }

    async fn get_books_by_authors(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetBooksByAuthorsQueryParams,
    ) -> Result<book::GetBooksByAuthorsResponse, ()> {
        match map_strings_to_ksuids(&query_params.authors) {
            Ok(ids) => match self.book_service.get_books_by_authors(ids).await {
                Ok(books) => {
                    let models = books.into_iter().map(map_book_to_rest).collect();
                    Ok(book::GetBooksByAuthorsResponse::Status200_SuccessfulOperation(models))
                }
                Err(_) => Ok(book::GetBooksByAuthorsResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::GetBooksByAuthorsResponse::Status400_InvalidAuthorValues),
        }
    }

    async fn get_books_by_generes(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetBooksByGeneresQueryParams,
    ) -> Result<book::GetBooksByGeneresResponse, ()> {
        match map_strings_to_ksuids(&query_params.genres) {
            Ok(ids) => match self.book_service.get_books_by_generes(ids).await {
                Ok(books) => {
                    let models = books.into_iter().map(map_book_to_rest).collect();
                    Ok(book::GetBooksByGeneresResponse::Status200_SuccessfulOperation(models))
                }
                Err(_) => Ok(book::GetBooksByGeneresResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::GetBooksByGeneresResponse::Status400_InvalidGenreValues),
        }
    }

    async fn get_books_by_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetBooksByStatusQueryParams,
    ) -> Result<book::GetBooksByStatusResponse, ()> {
        match map_book_status_list_to_domain(&query_params.status) {
            Ok(status_list) => match self.book_service.get_books_by_status(status_list).await {
                Ok(books) => {
                    let models = books.into_iter().map(map_book_to_rest).collect();
                    Ok(book::GetBooksByStatusResponse::Status200_SuccessfulOperation(models))
                }
                Err(_) => Ok(book::GetBooksByStatusResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::GetBooksByStatusResponse::Status400_InvalidStatusValue),
        }
    }

    async fn update_book(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateBookPathParams,
        body: &models::BookProperties,
    ) -> Result<book::UpdateBookResponse, ()> {
        match map_book_props_to_domain(&path_params.book_id, body) {
            Ok(props) => match self.book_service.update_book(props).await {
                Ok(book) => {
                    let model = map_book_to_rest(book);
                    Ok(book::UpdateBookResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(book::UpdateBookResponse::Status404_BookNotFound)
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(book::UpdateBookResponse::Status422_ValidationException)
                }
                Err(_) => Ok(book::UpdateBookResponse::Status500_ServerError),
            },
            Err(_) => Ok(book::UpdateBookResponse::Status400_InvalidParameters),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl discount::Discount for BookStoreServer {
    async fn add_discount(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewDiscountCode,
    ) -> Result<discount::AddDiscountResponse, ()> {
        match map_new_discount_code_to_domain(body) {
            Ok(new_discount) => match self.book_service.create_discount_code(new_discount).await {
                Ok(d) => {
                    let model = map_discount_code_to_rest(d);
                    Ok(discount::AddDiscountResponse::Status200_SuccessfulOperation(model))
                }
                Err(_) => Ok(discount::AddDiscountResponse::Status500_ServerError),
            },
            Err(_) => Ok(discount::AddDiscountResponse::Status400_InvalidInput),
        }
    }

    async fn delete_discount(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteDiscountPathParams,
    ) -> Result<discount::DeleteDiscountResponse, ()> {
        match Ksuid::from_str(&path_params.discount_id) {
            Ok(id) => match self.book_service.delte_discount_code_by_id(id).await {
                Ok(_) => Ok(discount::DeleteDiscountResponse::Status200_SuccessfulOperation),
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(discount::DeleteDiscountResponse::Status404_DiscontNotFound)
                }
                Err(_) => Ok(discount::DeleteDiscountResponse::Status500_ServerError),
            },
            Err(_) => Ok(discount::DeleteDiscountResponse::Status400_InvalidDiscountIdValue),
        }
    }

    async fn get_discount_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetDiscountByIdPathParams,
    ) -> Result<discount::GetDiscountByIdResponse, ()> {
        match Ksuid::from_str(&path_params.discount_id) {
            Ok(id) => match self.book_service.get_discount_code_by_id(id).await {
                Ok(d) => {
                    let model = map_discount_code_to_rest(d);
                    Ok(discount::GetDiscountByIdResponse::Status200_SuccessfulOperation(model))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(discount::GetDiscountByIdResponse::Status404_DiscountCodeNotFound)
                }
                Err(_) => Ok(discount::GetDiscountByIdResponse::Status500_ServerError),
            },
            Err(_) => Ok(discount::GetDiscountByIdResponse::Status400_InvalidParameters),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl store::Store for BookStoreServer {
    async fn delete_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteOrderPathParams,
    ) -> Result<store::DeleteOrderResponse, ()> {
        match Ksuid::from_str(&path_params.order_id) {
            Ok(order_id) => {
                // Now we have a valid Ksuid, we can use it with the order service
                match self.order_service.delete_order_by_id(order_id).await {
                    Ok(_) => Ok(store::DeleteOrderResponse::Status200_SuccessfulOperation),
                    Err(_) => Ok(store::DeleteOrderResponse::Status404_OrderNotFound),
                }
            }
            Err(_) => Ok(store::DeleteOrderResponse::Status400_InvalidIDSupplied),
        }
    }

    async fn get_inventory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<store::GetInventoryResponse, ()> {
        match self.order_service.get_inventory().await {
            Ok(inventory) => {
                let model = map_inventory_to_rest(inventory);
                Ok(store::GetInventoryResponse::Status200_SuccessfulOperation(
                    model,
                ))
            }
            Err(_) => Ok(store::GetInventoryResponse::Status500_ServerError),
        }
    }

    async fn get_order_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetOrderByIdPathParams,
    ) -> Result<store::GetOrderByIdResponse, ()> {
        match Ksuid::from_str(&path_params.order_id) {
            Ok(order_id) => match self.order_service.get_order_by_id(order_id).await {
                Ok(order) => {
                    let model = map_order_to_rest(order);
                    Ok(store::GetOrderByIdResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(store::GetOrderByIdResponse::Status400_InvalidIDSupplied)
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(store::GetOrderByIdResponse::Status404_OrderNotFound)
                }
                Err(domain::error::DomainError::FatalDBFailure { .. }) => {
                    Ok(store::GetOrderByIdResponse::Status500_ServerError)
                }
            },
            Err(_) => Ok(store::GetOrderByIdResponse::Status400_InvalidIDSupplied),
        }
    }

    async fn place_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewOrder,
    ) -> Result<store::PlaceOrderResponse, ()> {
        match map_new_order_to_domain(body) {
            Ok(domain) => match self.order_service.create_order(domain).await {
                Ok(result) => {
                    let model = map_order_to_rest(result);
                    Ok(store::PlaceOrderResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(store::PlaceOrderResponse::Status422_ValidationException)
                }
                Err(_) => Ok(store::PlaceOrderResponse::Status500_ServerError),
            },
            Err(_) => Ok(store::PlaceOrderResponse::Status400_InvalidInput),
        }
    }

    async fn update_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateOrderPathParams,
        body: &models::OrderProperties,
    ) -> Result<store::UpdateOrderResponse, ()> {
        match map_order_props_to_domain(path_params.order_id.as_str(), body) {
            Ok(domain) => match self.order_service.update_order(domain).await {
                Ok(result) => {
                    let model = map_order_to_rest(result);
                    Ok(store::UpdateOrderResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(store::UpdateOrderResponse::Status404_OrderNotFound)
                }
                Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                    Ok(store::UpdateOrderResponse::Status422_ValidationException)
                }
                Err(_) => Ok(store::UpdateOrderResponse::Status500_ServerError),
            },
            Err(_) => Ok(store::UpdateOrderResponse::Status400_InvalidParameters),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl genre::Genre for BookStoreServer {
    async fn add_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::NewGenre,
    ) -> Result<genre::AddGenreResponse, ()> {
        let new_genre = map_new_genre_to_domain(&body.name);
        match self.book_service.create_genre(new_genre).await {
            Ok(genre) => {
                let model = map_genre_to_rest(genre);
                Ok(genre::AddGenreResponse::Status200_SuccessfulOperation(
                    model,
                ))
            }
            Err(domain::error::DomainError::BusinessConstraintViolation { .. }) => {
                Ok(genre::AddGenreResponse::Status422_ValidationException)
            }
            Err(_) => Ok(genre::AddGenreResponse::Status500_ServerError),
        }
    }

    async fn delete_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteGenrePathParams,
    ) -> Result<genre::DeleteGenreResponse, ()> {
        match Ksuid::from_str(&path_params.genre_id) {
            Ok(id) => match self.book_service.delte_genre_by_id(id).await {
                Ok(_) => Ok(genre::DeleteGenreResponse::Status200_SuccessfulOperation),
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(genre::DeleteGenreResponse::Status404_GenreNotFound)
                }
                Err(_) => Ok(genre::DeleteGenreResponse::Status500_ServerError),
            },
            Err(_) => Ok(genre::DeleteGenreResponse::Status400_InvalidGenereIdValue),
        }
    }

    async fn get_genre_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetGenreByIdPathParams,
    ) -> Result<genre::GetGenreByIdResponse, ()> {
        match Ksuid::from_str(&path_params.genre_id) {
            Ok(id) => match self.book_service.get_genre_by_id(id).await {
                Ok(genre) => {
                    let model = map_genre_to_rest(genre);
                    Ok(genre::GetGenreByIdResponse::Status200_SuccessfulOperation(
                        model,
                    ))
                }
                Err(domain::error::DomainError::NotFound { .. }) => {
                    Ok(genre::GetGenreByIdResponse::Status404_GenrNotFound)
                }
                Err(_) => Ok(genre::GetGenreByIdResponse::Status500_ServerError),
            },
            Err(_) => Ok(genre::GetGenreByIdResponse::Status400_InvalidParameters),
        }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl ApiKeyAuthHeader for BookStoreServer {
    type Claims = bool;
    // TODO: check how to add API key auth as request interceptor, or offload to CloudFront/ALB
    async fn extract_claims_from_header(
        &self,
        headers: &HeaderMap,
        key: &str,
    ) -> Option<Self::Claims> {
        return Some(true);
    }
}
