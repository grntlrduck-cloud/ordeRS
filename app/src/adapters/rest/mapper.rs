use std::str::FromStr;

use crate::domain::models as dmodels;
use openapi::models as rmodels;
use svix_ksuid::Ksuid;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct OrderStatusError(String);

impl fmt::Display for OrderStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid order status: {}", self.0)
    }
}

impl Error for OrderStatusError {}

#[derive(Debug)]
pub enum MapperError {
    InvalidKsuid {
        id: String,
        source: svix_ksuid::Error,
    },
    InvalidOrderStatus {
        status: String,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapperError::InvalidKsuid { id, .. } => write!(f, "Invalid KSUID format: {}", id),
            MapperError::InvalidOrderStatus { status, .. } => {
                write!(f, "Invalid order status: {}", status)
            }
        }
    }
}

impl Error for MapperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MapperError::InvalidKsuid { source, .. } => Some(source),
            MapperError::InvalidOrderStatus { source, .. } => Some(source.as_ref()),
        }
    }
}

fn map_order_to_domain(order: rmodels::Order) -> Result<dmodels::OrderDomain, MapperError> {
    let order_id = Ksuid::from_str(&order.id).map_err(|e| MapperError::InvalidKsuid {
        id: order.id.clone(),
        source: e,
    })?;
    let book_id = Ksuid::from_str(&order.book_id).map_err(|e| MapperError::InvalidKsuid {
        id: order.book_id.clone(),
        source: e,
    })?;
    let customer_id =
        Ksuid::from_str(&order.customer_id).map_err(|e| MapperError::InvalidKsuid {
            id: order.customer_id.clone(),
            source: e,
        })?;

    let shipping_address = match order.shipping_address_override {
        Some(address) => map_address_to_domain(address),
        None => map_address_to_domain(order.billing_address.clone()),
    };

    let status = dmodels::OrderStatus::from_str(&order.status).map_err(|_| {
        MapperError::InvalidOrderStatus {
            status: order.status.clone(),
            source: Box::new(OrderStatusError(order.status.clone())),
        }
    })?;

    Ok(dmodels::OrderDomain {
        id: order_id,
        book_id,
        customer_id,
        quantity: order.quantity,
        shipping_date: order.shipping_date,
        billing_address: map_address_to_domain(order.billing_address.clone()),
        shipping_address,
        status,
    })
}

fn map_address_to_domain(address: rmodels::Address) -> dmodels::AddressDomain {
    dmodels::AddressDomain {
        street: address.street,
        stree_number: address.street_number,
        zip_code: address.zip_code,
        city: address.city,
        province: address.province,
        country: address.country,
    }
}
