use std::str::FromStr;

use crate::domain::models as dmodels;
use openapi::models as rmodels;
use svix_ksuid::Ksuid;

#[derive(Debug)]
pub enum MapperError {
    InvalidKsuid(String),
    InvalidOrderStatus(String),
}

fn map_order_to_domain(order: rmodels::Order) -> Result<dmodels::OrderDomain, MapperError> {
    let order_id =
        Ksuid::from_str(&order.id).map_err(|_| MapperError::InvalidKsuid(order.id.clone()))?;
    let book_id = Ksuid::from_str(&order.book_id)
        .map_err(|_| MapperError::InvalidKsuid(order.book_id.clone()))?;
    let customer_id = Ksuid::from_str(&order.customer_id)
        .map_err(|_| MapperError::InvalidKsuid(order.customer_id.clone()))?;

    let shipping_address = match order.shipping_address_override {
        Some(address) => map_address_to_domain(address),
        None => map_address_to_domain(order.billing_address.clone()),
    };

    let status = dmodels::OrderStatus::from_str(&order.status)
        .map_err(|_| MapperError::InvalidOrderStatus(order.status.clone()))?;

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
