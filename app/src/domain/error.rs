use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OrderNotFoundError(pub String);

impl fmt::Display for OrderNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Order not found: {}", self.0)
    }
}

impl Error for OrderNotFoundError {}

#[derive(Debug)]
pub struct BookNotFoundError(pub String);

impl fmt::Display for BookNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Book not found: {}", self.0)
    }
}

impl Error for BookNotFoundError {}

#[derive(Debug)]
pub struct AuthorNotFoundError(pub String);

impl fmt::Display for AuthorNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Author not found: {}", self.0)
    }
}

impl Error for AuthorNotFoundError {}

#[derive(Debug)]
pub struct GenreNotFoundError(pub String);

impl fmt::Display for GenreNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Genre not found: {}", self.0)
    }
}

impl Error for GenreNotFoundError {}

#[derive(Debug)]
pub struct DiscountCodeNotFoundError(pub String);

impl fmt::Display for DiscountCodeNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DiscountCode not found: {}", self.0)
    }
}

impl Error for DiscountCodeNotFoundError {}

#[derive(Debug)]
pub enum DomainError {
    NotFoundError {
        id: String,
        source: Box<dyn Error + Send + Sync>,
    },
    FatalDBError {
        message: String,
        source: Box<dyn Error + Send + Sync>,
    },
    BusinessValidationError {
        message: String,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFoundError { id, .. } => write!(f, "Item not found: {}", id),
            DomainError::FatalDBError { message, .. } => {
                write!(f, "Failed to perform DB operation: {}", message)
            }
            DomainError::BusinessValidationError { message, .. } => {
                write!(f, "Business validation constraints not met: {}", message)
            }
        }
    }
}

impl Error for DomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DomainError::NotFoundError { source, .. } => Some(source.as_ref()),
            DomainError::FatalDBError { source, .. } => Some(source.as_ref()),
            DomainError::BusinessValidationError { source, .. } => Some(source.as_ref()),
        }
    }
}
