use async_trait::async_trait;
use svix_ksuid::Ksuid;

use super::error;
use super::models;

/// The OrderHandler takes care of the order inflow and inventory.
/// Loose couling to the other entities handled by the application.
#[async_trait]
pub trait OrderHandler {
    /// Get an existing order by id
    async fn get_order_by_id(&self, id: Ksuid) -> Result<models::OrderDomain, error::DomainError>;

    /// Delete an existing order by id
    async fn delete_order_by_id(&self, id: Ksuid) -> Result<(), error::DomainError>;

    /// Get inventory statistics
    async fn get_inventory(&self) -> Result<models::InventoryDomain, error::DomainError>;

    /// Create a new book order
    async fn create_order(
        &self,
        order: models::OrderDomain,
    ) -> Result<models::OrderDomain, error::DomainError>;

    /// Update an existing order
    async fn update_order(
        &self,
        props: models::OrderUpdateProps,
    ) -> Result<models::OrderDomain, error::DomainError>;
}

/// The BookStore handles the request related to the books and related entities.
/// Books, Authors, Genres, and DiscountCodes are grouped together due to tight coupling
#[async_trait]
pub trait BookHandler {
    // the book functions
    /// Create a new book in the store
    async fn create_book(
        &self,
        book: models::NewBookDomain,
    ) -> Result<models::BookDomain, error::DomainError>;

    /// Delete an existing book in the store
    async fn delete_book_by_id(&self, id: Ksuid) -> Result<(), error::DomainError>;

    /// Get an existing book by id
    async fn get_book_by_id(&self, id: Ksuid) -> Result<models::BookDomain, error::DomainError>;

    /// get all books in the list of authors
    async fn get_books_by_authors(
        &self,
        authors: Vec<Ksuid>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError>;

    /// Get all books with matching at least one genre
    async fn get_books_by_generes(
        &self,
        genres: Vec<Ksuid>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError>;

    /// Get all books matching one of the status given in the list
    async fn get_books_by_status(
        &self,
        status: Vec<models::BookStatus>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError>;

    /// Update an existing book in the store
    async fn update_book(
        &self,
        props: models::BookUpdateProps,
    ) -> Result<models::BookDomain, error::DomainError>;

    // discount code functions
    /// Create a new discount code in the store
    async fn create_discount_code(
        &self,
        discount_code: models::DiscountCodeDomain,
    ) -> Result<models::DiscountCodeDomain, error::DomainError>;

    /// Delete an existing discount code
    async fn delte_discount_code_by_id(&self, id: Ksuid) -> Result<(), error::DomainError>;

    /// Get a discount code by id
    async fn get_discount_code_by_id(
        &self,
        id: Ksuid,
    ) -> Result<models::DiscountCodeDomain, error::DomainError>;

    // gerne functions
    /// Create a new genre in the book store
    async fn create_genre(
        &self,
        genre: models::GenereDomain,
    ) -> Result<models::GenereDomain, error::DomainError>;

    /// Delete an existing genre in the store
    async fn delte_genre_by_id(&self, id: Ksuid) -> Result<(), error::DomainError>;

    /// Get an existing genre by id
    async fn get_genre_by_id(&self, id: Ksuid) -> Result<models::GenereDomain, error::DomainError>;

    // author functions
    /// Create a new book author
    async fn create_author(
        &self,
        author: models::AuthorDomain,
    ) -> Result<models::AuthorDomain, error::DomainError>;

    /// Delete an existing author
    async fn delte_author_by_id(&self, id: Ksuid) -> Result<(), error::DomainError>;

    /// Get an existing author by id
    async fn get_author_by_id(&self, id: Ksuid)
    -> Result<models::AuthorDomain, error::DomainError>;

    /// Update an existing author
    async fn update_author(
        &self,
        props: models::AuthorUpdateProps,
    ) -> Result<models::AuthorDomain, error::DomainError>;
}
