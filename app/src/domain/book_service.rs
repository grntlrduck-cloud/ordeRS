use std::sync::Arc;

use super::error;
use super::models;
use super::store;
use async_trait::async_trait;
use chrono::Days;
use chrono::NaiveDate;
use chrono::Utc;
use svix_ksuid::Ksuid;
use svix_ksuid::KsuidLike;

pub struct BookService {
    todo: String,
}

impl BookService {
    pub fn new(todo: String) -> Arc<Self> {
        Arc::new(BookService { todo })
    }
}

#[async_trait]
impl store::BookHandler for BookService {
    /// Create a new book in the store
    async fn create_book(
        &self,
        book: models::NewBookDomain,
    ) -> Result<models::BookDomain, error::DomainError> {
        Ok(models::BookDomain {
            authors: vec![],
            available: book.available,
            discounts: None,
            edition: book.edition,
            firs_release: book.first_release,
            genres: None,
            id: book.id,
            price: book.price,
            release: book.release,
            series: book.series,
            status: book.status,
            title: book.title,
        })
    }

    /// Delete an existing book in the store
    async fn delete_book_by_id(&self, id: Ksuid) -> Result<(), error::DomainError> {
        Err(error::DomainError::NotFound {
            id: id.to_string(),
            source: Box::new(error::BookNotFoundError(id.to_string())),
        })
    }

    /// Get an existing book by id
    async fn get_book_by_id(&self, id: Ksuid) -> Result<models::BookDomain, error::DomainError> {
        Ok(models::BookDomain {
            authors: vec![],
            available: 2,
            discounts: None,
            edition: 1,
            firs_release: Utc::now().date_naive(),
            genres: None,
            id,
            price: 12.5,
            release: Utc::now().date_naive(),
            series: Some(String::from("1")),
            status: models::BookStatus::Available,
            title: String::from("The best book"),
        })
    }

    /// get all books in the list of authors
    async fn get_books_by_authors(
        &self,
        authors: Vec<Ksuid>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError> {
        Ok(vec![models::BookDomain {
            authors: vec![],
            available: 2,
            discounts: None,
            edition: 1,
            firs_release: Utc::now().naive_utc().date(),
            genres: None,
            id: Ksuid::new(None, None),
            price: 12.5,
            release: Utc::now().naive_utc().date(),
            series: Some(String::from("1")),
            status: models::BookStatus::Available,
            title: String::from("The best book"),
        }])
    }

    /// Get all books with matching at least one genre
    async fn get_books_by_generes(
        &self,
        genres: Vec<Ksuid>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError> {
        Ok(vec![models::BookDomain {
            authors: vec![],
            available: 2,
            discounts: None,
            edition: 1,
            firs_release: Utc::now().naive_utc().date(),
            genres: None,
            id: Ksuid::new(None, None),
            price: 12.5,
            release: Utc::now().naive_utc().date(),
            series: Some(String::from("1")),
            status: models::BookStatus::Available,
            title: String::from("The best book"),
        }])
    }

    /// Get all books matching one of the status given in the list
    async fn get_books_by_status(
        &self,
        status: Vec<models::BookStatus>,
    ) -> Result<Vec<models::BookDomain>, error::DomainError> {
        Ok(vec![models::BookDomain {
            authors: vec![],
            available: 2,
            discounts: None,
            edition: 1,
            firs_release: Utc::now().date_naive(),
            genres: None,
            id: Ksuid::new(None, None),
            price: 12.5,
            release: Utc::now().naive_utc().date(),
            series: Some(String::from("1")),
            status: models::BookStatus::Available,
            title: String::from("The best book"),
        }])
    }

    /// Update an existing book in the store
    async fn update_book(
        &self,
        props: models::BookUpdateProps,
    ) -> Result<models::BookDomain, error::DomainError> {
        Ok(models::BookDomain {
            authors: vec![],
            available: 2,
            discounts: None,
            edition: 1,
            firs_release: Utc::now().date_naive(),
            genres: None,
            id: props.id,
            price: 12.5,
            release: Utc::now().date_naive(),
            series: Some(String::from("1")),
            status: models::BookStatus::Available,
            title: String::from("The best book"),
        })
    }

    // discount code functions
    /// Create a new discount code in the store
    async fn create_discount_code(
        &self,
        discount_code: models::DiscountCodeDomain,
    ) -> Result<models::DiscountCodeDomain, error::DomainError> {
        Ok(discount_code)
    }

    /// Delete an existing discount code
    async fn delte_discount_code_by_id(&self, id: Ksuid) -> Result<(), error::DomainError> {
        Err(error::DomainError::NotFound {
            id: id.to_string(),
            source: Box::new(error::DiscountCodeNotFoundError(id.to_string())),
        })
    }

    /// Get a discount code by id
    async fn get_discount_code_by_id(
        &self,
        id: Ksuid,
    ) -> Result<models::DiscountCodeDomain, error::DomainError> {
        Ok(models::DiscountCodeDomain {
            code: String::from("CODE25"),
            id,
            percentage_discount: 25,
            valid_from: Utc::now().date_naive(),
            valid_to: Utc::now()
                .date_naive()
                .checked_add_days(Days::new(30))
                .unwrap(),
        })
    }

    // gerne functions
    /// Create a new genre in the book store
    async fn create_genre(
        &self,
        genre: models::GenereDomain,
    ) -> Result<models::GenereDomain, error::DomainError> {
        Ok(genre)
    }

    /// Delete an existing genre in the store
    async fn delte_genre_by_id(&self, id: Ksuid) -> Result<(), error::DomainError> {
        Err(error::DomainError::BusinessConstraintViolation {
            message: format!("failed to delete genre {}", id),
            source: Box::new(error::GenreNotFoundError(id.to_string())),
        })
    }

    /// Get an existing genre by id
    async fn get_genre_by_id(&self, id: Ksuid) -> Result<models::GenereDomain, error::DomainError> {
        Ok(models::GenereDomain {
            id,
            name: String::from("Horror"),
        })
    }

    // author functions
    /// Create a new book author
    async fn create_author(
        &self,
        author: models::AuthorDomain,
    ) -> Result<models::AuthorDomain, error::DomainError> {
        Ok(author)
    }

    /// Delete an existing author
    async fn delte_author_by_id(&self, id: Ksuid) -> Result<(), error::DomainError> {
        Err(error::DomainError::FatalDBFailure {
            message: format!("Failed to delete author {}", id),
            source: Box::new(error::AuthorNotFoundError(id.to_string())),
        })
    }

    /// Get an existing author by id
    async fn get_author_by_id(
        &self,
        id: Ksuid,
    ) -> Result<models::AuthorDomain, error::DomainError> {
        Ok(models::AuthorDomain {
            date_of_birth: NaiveDate::from_ymd_opt(1920, 06, 06).unwrap(),
            date_of_death: Some(NaiveDate::from_ymd_opt(1980, 12, 12).unwrap()),
            first_name: String::from("Hans"),
            id,
            last_name: String::from("Wurst"),
            second_names: Some(vec![String::from("Weiss")]),
            title: None,
        })
    }

    /// Update an existing author
    async fn update_author(
        &self,
        props: models::AuthorUpdateProps,
    ) -> Result<models::AuthorDomain, error::DomainError> {
        Ok(models::AuthorDomain {
            date_of_birth: NaiveDate::from_ymd_opt(1980, 12, 12).unwrap(),
            date_of_death: NaiveDate::from_ymd_opt(2024, 12, 13),
            first_name: String::from("Hans"),
            id: props.id,
            last_name: String::from("Wurst"),
            second_names: Some(vec![String::from("Weiss")]),
            title: None,
        })
    }
}
