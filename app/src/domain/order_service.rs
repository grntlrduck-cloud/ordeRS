use super::{error, models, store};
use async_trait::async_trait;
use chrono::Utc;
use svix_ksuid::{Ksuid, KsuidLike};

pub struct OrderService {
    todo: String,
}

impl OrderService {
    pub fn new(todo: String) -> Self {
        OrderService { todo }
    }
}

#[async_trait]
impl store::OrderHandler for OrderService {
    /// Get an existing order by id
    async fn get_order_by_id(&self, id: Ksuid) -> Result<models::OrderDomain, error::DomainError> {
        let address = models::AddressDomain {
            street: String::from("Street"),
            street_number: String::from("1b"),
            city: String::from("My City"),
            zip_code: String::from("123456"),
            province: Some(String::from("province")),
            country: String::from("Country"),
        };
        let books = vec![models::OrderedBookDomain {
            book_id: Ksuid::new(None, None),
            quantity: 8,
        }];
        Ok(models::OrderDomain {
            billing_address: address.clone(),
            books,
            customer_id: Ksuid::new(None, None),
            id,
            shipping_address: address,
            shipping_date: Utc::now().date_naive(),
            status: models::OrderStatus::Shipped,
        })
    }

    /// Delete an existing order by id
    async fn delete_order_by_id(&self, id: Ksuid) -> Result<(), error::DomainError> {
        Ok(())
    }

    /// Get inventory statistics
    async fn get_inventory(&self) -> Result<models::InventoryDomain, error::DomainError> {
        Ok(models::InventoryDomain {
            books_available: 42,
            books_reordered: 42,
            books_out_of_stock: 42,
        })
    }

    /// Create a new book order
    async fn create_order(
        &self,
        order: models::OrderDomain,
    ) -> Result<models::OrderDomain, error::DomainError> {
        Ok(order.clone())
    }

    /// Update an existing order
    async fn update_order(
        &self,
        props: models::OrderUpdateProps,
    ) -> Result<models::OrderDomain, error::DomainError> {
        let address = models::AddressDomain {
            street: String::from("Street"),
            street_number: String::from("1b"),
            city: String::from("My City"),
            zip_code: String::from("123456"),
            province: Some(String::from("province")),
            country: String::from("Country"),
        };
        let books = vec![models::OrderedBookDomain {
            book_id: Ksuid::new(None, None),
            quantity: 8,
        }];
        Ok(models::OrderDomain {
            billing_address: address.clone(),
            books,
            customer_id: Ksuid::new(None, None),
            id: props.id,
            shipping_address: address,
            shipping_date: Utc::now().date_naive(),
            status: models::OrderStatus::Shipped,
        })
    }
}
