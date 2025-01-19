use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BookAvailabilityError(pub i32);

impl fmt::Display for BookAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number of books available: {}", self.0)
    }
}

impl Error for BookAvailabilityError {}

#[derive(Debug)]
pub struct BookStatusError(pub String);

impl fmt::Display for BookStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid book status: {}", self.0)
    }
}

impl Error for BookStatusError {}

#[derive(Debug)]
pub struct DiscountPercentageError(pub i32);

impl fmt::Display for DiscountPercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Discount percentage {} exceeds maximum allowed value of 80%",
            self.0
        )
    }
}

impl Error for DiscountPercentageError {}

#[derive(Debug)]
pub struct OrderStatusError(pub String);

impl fmt::Display for OrderStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid order status: {}", self.0)
    }
}

impl Error for OrderStatusError {}

#[derive(Debug)]
pub struct OrderQuantityError(pub i32);

impl fmt::Display for OrderQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid order quantity: {}", self.0)
    }
}

impl Error for OrderQuantityError {}

#[derive(Debug)]
pub enum MapperError {
    InvalidKsuid {
        id: String,
        source: svix_ksuid::Error,
    },
    BooksAvailableOutOfBound {
        books_available: i32,
        source: Box<dyn Error + Send + Sync>,
    },
    DiscountPercentageOutOfBounds {
        percentage: i32,
        source: Box<dyn Error + Send + Sync>,
    },
    InvalidBookStatus {
        status: String,
        source: Box<dyn Error + Send + Sync>,
    },
    InvalidOrderStatus {
        status: String,
        source: Box<dyn Error + Send + Sync>,
    },
    OrderQuantityOutOfBounds {
        quantity: i32,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapperError::InvalidKsuid { id, .. } => write!(f, "Invalid KSUID format: {}", id),
            MapperError::BooksAvailableOutOfBound {
                books_available, ..
            } => {
                write!(f, "Invalid order status: {}", books_available)
            }
            MapperError::DiscountPercentageOutOfBounds { percentage, .. } => {
                write!(
                    f,
                    "Invalid discount percentage: {}. Maximum allowed is 80%",
                    percentage
                )
            }
            MapperError::InvalidBookStatus { status, .. } => {
                write!(f, "Invalid book status: {}", status)
            }
            MapperError::InvalidOrderStatus { status, .. } => {
                write!(f, "Invalid order status: {}", status)
            }
            MapperError::OrderQuantityOutOfBounds { quantity, .. } => {
                write!(f, "Invalid quantity for order: {}. Minimum is 1", quantity)
            }
        }
    }
}

impl Error for MapperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MapperError::InvalidKsuid { source, .. } => Some(source),
            MapperError::BooksAvailableOutOfBound { source, .. } => Some(source.as_ref()),
            MapperError::DiscountPercentageOutOfBounds { source, .. } => Some(source.as_ref()),
            MapperError::InvalidBookStatus { source, .. } => Some(source.as_ref()),
            MapperError::InvalidOrderStatus { source, .. } => Some(source.as_ref()),
            MapperError::OrderQuantityOutOfBounds { source, .. } => Some(source.as_ref()),
        }
    }
}
