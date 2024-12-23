use async_trait::async_trait;
use axum::extract::*;
use axum::http::header::HeaderMap;
use axum_extra::extract::CookieJar;
use http::Method;
use openapi::apis::{author, book, discount, health, store, ApiKeyAuthHeader};
use openapi::models;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber;

/// TODO: implement function bodies
/// here will come the implementation of the API handler
pub struct BookStoreServer {
    // TODO: add db layer
}

pub async fn start_server(addr: &str) {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Init Axum router
    let app = openapi::server::new(Arc::new(BookStoreServer {}));

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
        // TODO:: handle shut down
        _ = ctrl_c => println!("Received ctrl + c"),
        _ = terminate => println!("Received terminate"),
    }
}

#[allow(unused_variables)]
#[async_trait]
impl health::Health for BookStoreServer {
    async fn get_readiness(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<health::GetReadinessResponse, ()> {
        Ok(
            health::GetReadinessResponse::Status200_TheHealthCheckReadinessResponses(
                models::HealthCheckResponse {
                    status: Some(String::from("serving")),
                },
            ),
        )
    }
}

#[allow(unused_variables)]
#[async_trait]
impl author::Author for BookStoreServer {
    async fn add_author(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NewAuthor,
    ) -> Result<author::AddAuthorResponse, ()> {
        Ok(author::AddAuthorResponse::Status400_InvalidInput)
    }

    async fn delete_author(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteAuthorPathParams,
    ) -> Result<author::DeleteAuthorResponse, ()> {
        Ok(author::DeleteAuthorResponse::Status400_InvalidAuthorIdValue)
    }

    async fn get_author_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetAuthorByIdPathParams,
    ) -> Result<author::GetAuthorByIdResponse, ()> {
        println!("Not yet implemented!");
        Ok(author::GetAuthorByIdResponse::Status400_InvalidParameters)
    }

    async fn update_author(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateAuthorPathParams,
        body: models::AuthorProperties,
    ) -> Result<author::UpdateAuthorResponse, ()> {
        Ok(author::UpdateAuthorResponse::Status400_InvalidParameters)
    }
}

#[allow(unused_variables)]
#[async_trait]
impl book::Book for BookStoreServer {
    async fn add_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NewBook,
    ) -> Result<book::AddBookResponse, ()> {
        Ok(book::AddBookResponse::Status400_InvalidInput)
    }

    async fn delete_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteBookPathParams,
    ) -> Result<book::DeleteBookResponse, ()> {
        Ok(book::DeleteBookResponse::Status400_InvalidBookIdValue)
    }

    async fn get_book_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetBookByIdPathParams,
    ) -> Result<book::GetBookByIdResponse, ()> {
        Ok(book::GetBookByIdResponse::Status400_InvalidParameters)
    }

    async fn get_books_by_authors(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByAuthorsQueryParams,
    ) -> Result<book::GetBooksByAuthorsResponse, ()> {
        Ok(book::GetBooksByAuthorsResponse::Status400_InvalidAuthorValues)
    }

    async fn get_books_by_generes(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByGeneresQueryParams,
    ) -> Result<book::GetBooksByGeneresResponse, ()> {
        Ok(book::GetBooksByGeneresResponse::Status400_InvalidGenreValues)
    }

    async fn get_books_by_status(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetBooksByStatusQueryParams,
    ) -> Result<book::GetBooksByStatusResponse, ()> {
        Ok(book::GetBooksByStatusResponse::Status400_InvalidStatusValue)
    }

    async fn update_book(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateBookPathParams,
        body: models::BookProperties,
    ) -> Result<book::UpdateBookResponse, ()> {
        Ok(book::UpdateBookResponse::Status400_InvalidParameters)
    }
}

#[allow(unused_variables)]
#[async_trait]
impl discount::Discount for BookStoreServer {
    async fn add_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NewDiscountCode,
    ) -> Result<discount::AddDiscountResponse, ()> {
        Ok(discount::AddDiscountResponse::Status400_InvalidInput)
    }

    async fn delete_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteDiscountPathParams,
    ) -> Result<discount::DeleteDiscountResponse, ()> {
        Ok(discount::DeleteDiscountResponse::Status400_InvalidDiscountIdValue)
    }

    async fn get_discount_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetDiscountByIdPathParams,
    ) -> Result<discount::GetDiscountByIdResponse, ()> {
        Ok(discount::GetDiscountByIdResponse::Status400_InvalidParameters)
    }

    async fn update_discount(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateDiscountPathParams,
        body: models::DiscountCodeProperties,
    ) -> Result<discount::UpdateDiscountResponse, ()> {
        Ok(discount::UpdateDiscountResponse::Status400_InvalidIDSupplied)
    }
}

#[allow(unused_variables)]
#[async_trait]
impl store::Store for BookStoreServer {
    async fn delete_order(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::DeleteOrderPathParams,
    ) -> Result<store::DeleteOrderResponse, ()> {
        Ok(store::DeleteOrderResponse::Status400_InvalidIDSupplied)
    }

    async fn get_inventory(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<store::GetInventoryResponse, ()> {
        Ok(store::GetInventoryResponse::Status500_ServerError)
    }

    async fn get_order_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetOrderByIdPathParams,
    ) -> Result<store::GetOrderByIdResponse, ()> {
        Ok(store::GetOrderByIdResponse::Status400_InvalidIDSupplied)
    }

    async fn place_order(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Option<models::NewOrder>,
    ) -> Result<store::PlaceOrderResponse, ()> {
        Ok(store::PlaceOrderResponse::Status400_InvalidInput)
    }

    async fn update_order(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UpdateOrderPathParams,
        body: models::OrderProperties,
    ) -> Result<store::UpdateOrderResponse, ()> {
        Ok(store::UpdateOrderResponse::Status400_InvalidParameters)
    }
}

#[allow(unused_variables)]
#[async_trait]
impl ApiKeyAuthHeader for BookStoreServer {
    type Claims = bool;
    // TODO: check how to add API key auth as request interceptor
    async fn extract_claims_from_header(
        &self,
        headers: &HeaderMap,
        key: &str,
    ) -> Option<Self::Claims> {
        return Some(true);
    }
}
