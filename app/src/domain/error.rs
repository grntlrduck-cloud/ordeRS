use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct OrderNotFoundError(String);

impl fmt::Display for OrderNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Order not found: {}", self.0)
    }
}

impl Error for OrderNotFoundError {}

#[derive(Debug)]
pub enum DomainError {
    NotFoundError {
        id: String,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFoundError { id, .. } => write!(f, "Item not found: {}", id),
        }
    }
}

impl Error for DomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DomainError::NotFoundError { source, .. } => Some(source.as_ref()),
        }
    }
}
