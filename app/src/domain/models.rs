use core::f64;

use svix_ksuid::Ksuid;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountCodeDomain {
    pub id: Ksuid,
    pub percentage_discount: i32,
    pub valid_from: chrono::naive::NaiveDate,
    pub valid_to: chrono::naive::NaiveDate,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountCodeUpdateProps {
    pub percentage_discount: Option<i32>,
    pub valid_from: Option<chrono::naive::NaiveDate>,
    pub valid_to: Option<chrono::naive::NaiveDate>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BookDomain {
    pub id: Ksuid,
    pub title: String,
    pub release: chrono::naive::NaiveDate,
    pub firs_release: chrono::naive::NaiveDate,
    pub authors: Vec<AuthorDomain>,
    pub series: Option<String>,
    pub edition: i32,
    pub price: f64,
    pub discounts: Option<Vec<DiscountCodeDomain>>,
    pub available: i32,
    pub status: BookStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BookUpdateProps {
    pub title: Option<String>,
    pub release: Option<chrono::naive::NaiveDate>,
    pub firs_release: Option<chrono::naive::NaiveDate>,
    pub authors: Option<Vec<AuthorDomain>>,
    pub series: Option<String>,
    pub edition: Option<i32>,
    pub price: Option<f64>,
    pub discounts: Option<Vec<DiscountCodeDomain>>,
    pub available: Option<i32>,
    pub status: Option<BookStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BookStatus {
    Available,
    ReOrdered,
    OutOfStock,
}

impl std::fmt::Display for BookStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookStatus::Available => write!(f, "available"),
            BookStatus::ReOrdered => write!(f, "re-ordered"),
            BookStatus::OutOfStock => write!(f, "out-of-stock"),
        }
    }
}

impl std::str::FromStr for BookStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available" => Ok(BookStatus::Available),
            "re-ordered" => Ok(BookStatus::ReOrdered),
            "out-of-stock" => Ok(BookStatus::OutOfStock),
            _ => Err(format!("Invalid book status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorDomain {
    pub id: Ksuid,
    pub title: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: chrono::naive::NaiveDate,
    pub date_of_death: Option<chrono::naive::NaiveDate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorUpdateProps {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<chrono::naive::NaiveDate>,
    pub date_of_death: Option<chrono::naive::NaiveDate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryDomain {
    pub books_available: i32,
    pub books_reordered: i32,
    pub books_out_of_stock: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderDomain {
    pub id: Ksuid,
    pub book_id: Ksuid,
    pub customer_id: Ksuid,
    pub quantity: i32,
    pub shipping_date: chrono::DateTime<chrono::Utc>,
    pub shipping_address: AddressDomain,
    pub billing_address: AddressDomain,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderUpdateProps {
    pub quantity: Option<i32>,
    pub shipping_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddressDomain {
    pub street: String,
    pub stree_number: String,
    pub zip_code: String,
    pub city: String,
    pub province: Option<String>,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Placed,
    Shipped,
    Delivered,
    Canceled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Placed => write!(f, "placed"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Canceled => write!(f, "canceled"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "placed" => Ok(OrderStatus::Placed),
            "shipped" => Ok(OrderStatus::Shipped),
            "delivered" => Ok(OrderStatus::Delivered),
            "canceled" => Ok(OrderStatus::Canceled),
            _ => Err(format!("Invalid order status: {}", s)),
        }
    }
}
