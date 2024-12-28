use core::f64;
use svix_ksuid::Ksuid;

#[derive(Debug, Clone, PartialEq)]
pub struct AddressDomain {
    pub city: String,
    pub country: String,
    pub province: Option<String>,
    pub street: String,
    pub street_number: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorDomain {
    pub date_of_birth: chrono::naive::NaiveDate,
    pub date_of_death: Option<chrono::naive::NaiveDate>,
    pub first_name: String,
    pub id: Ksuid,
    pub last_name: String,
    pub second_names: Option<Vec<String>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorUpdateProps {
    pub id: Ksuid,
    pub date_of_death: Option<chrono::naive::NaiveDate>,
    pub last_name: Option<String>,
    pub second_names: Option<Vec<String>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BookDomain {
    pub authors: Vec<AuthorDomain>,
    pub available: i32,
    pub discounts: Option<Vec<DiscountCodeDomain>>,
    pub edition: i32,
    pub firs_release: chrono::naive::NaiveDate,
    pub genres: Option<Vec<GenereDomain>>,
    pub id: Ksuid,
    pub price: f64,
    pub release: chrono::naive::NaiveDate,
    pub series: Option<String>,
    pub status: BookStatus,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BookStatus {
    Available,
    OutOfStock,
    ReOrdered,
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
pub struct BookUpdateProps {
    pub id: Ksuid,
    pub authors: Option<Vec<Ksuid>>,
    pub available: Option<i32>,
    pub discounts: Option<Vec<Ksuid>>,
    pub genres: Option<Vec<Ksuid>>,
    pub edition: Option<i32>,
    pub price: Option<f64>,
    pub release: Option<chrono::naive::NaiveDate>,
    pub series: Option<String>,
    pub status: Option<BookStatus>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountCodeDomain {
    pub code: String,
    pub id: Ksuid,
    pub percentage_discount: i32,
    pub valid_from: chrono::naive::NaiveDate,
    pub valid_to: chrono::naive::NaiveDate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenereDomain {
    pub id: Ksuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryDomain {
    pub books_available: i32,
    pub books_out_of_stock: i32,
    pub books_reordered: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewBookDomain {
    pub authors: Vec<Ksuid>,
    pub available: i32,
    pub discounts: Option<Vec<Ksuid>>,
    pub edition: i32,
    pub first_release: chrono::naive::NaiveDate,
    pub genres: Option<Vec<Ksuid>>,
    pub id: Ksuid,
    pub price: f64,
    pub release: chrono::naive::NaiveDate,
    pub series: Option<String>,
    pub status: BookStatus,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderDomain {
    pub billing_address: AddressDomain,
    pub customer_id: Ksuid,
    pub id: Ksuid,
    pub books: Vec<OrderedBookDomain>,
    pub shipping_address: AddressDomain,
    pub shipping_date: chrono::naive::NaiveDate,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderedBookDomain {
    pub book_id: Ksuid,
    pub quantity: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Canceled,
    Delivered,
    Placed,
    Shipped,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Canceled => write!(f, "canceled"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Placed => write!(f, "placed"),
            OrderStatus::Shipped => write!(f, "shipped"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "canceled" => Ok(OrderStatus::Canceled),
            "delivered" => Ok(OrderStatus::Delivered),
            "placed" => Ok(OrderStatus::Placed),
            "shipped" => Ok(OrderStatus::Shipped),
            _ => Err(format!("Invalid order status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderUpdateProps {
    pub id: Ksuid,
    pub shipping_date: chrono::DateTime<chrono::Utc>,
    pub status: OrderStatus,
}
