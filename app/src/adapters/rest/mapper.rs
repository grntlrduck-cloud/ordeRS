use crate::domain::models as dmodels;
use openapi::models as rmodels;
use svix_ksuid::*;

use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct BookStatusError(String);

impl fmt::Display for BookStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid book status: {}", self.0)
    }
}

impl Error for BookStatusError {}

#[derive(Debug)]
struct BookAvailabilityError(i32);

impl fmt::Display for BookAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number of books available: {}", self.0)
    }
}

impl Error for BookAvailabilityError {}

#[derive(Debug)]
pub enum MapperError {
    InvalidKsuid {
        id: String,
        source: svix_ksuid::Error,
    },
    InvalidBooksAvailable {
        books_available: i32,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapperError::InvalidKsuid { id, .. } => write!(f, "Invalid KSUID format: {}", id),
            MapperError::InvalidBooksAvailable {
                books_available, ..
            } => {
                write!(f, "Invalid order status: {}", books_available)
            }
        }
    }
}

impl Error for MapperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MapperError::InvalidKsuid { source, .. } => Some(source),
            MapperError::InvalidBooksAvailable { source, .. } => Some(source.as_ref()),
        }
    }
}

fn map_order_to_rest(order: dmodels::OrderDomain) -> rmodels::Order {
    let address_override = if order.billing_address == order.shipping_address {
        None
    } else {
        Some(map_address_to_rest(order.shipping_address))
    };
    rmodels::Order {
        id: order.id.to_string(),
        book_id: order.book_id.to_string(),
        customer_id: order.customer_id.to_string(),
        quantity: order.quantity,
        shipping_date: order.shipping_date,
        billing_address: map_address_to_rest(order.billing_address.clone()),
        shipping_address_override: address_override,
        status: order.status.to_string(),
    }
}

fn map_address_to_rest(address: dmodels::AddressDomain) -> rmodels::Address {
    rmodels::Address {
        street: address.street,
        street_number: address.street_number,
        zip_code: address.zip_code,
        city: address.city,
        province: address.province,
        country: address.country,
    }
}

fn map_new_order_to_domain(
    new_order: rmodels::NewOrder,
) -> Result<dmodels::OrderDomain, MapperError> {
    let book_id = Ksuid::from_str(&new_order.book_id).map_err(|e| MapperError::InvalidKsuid {
        id: new_order.book_id.clone(),
        source: e,
    })?;

    let customer_id =
        Ksuid::from_str(&new_order.customer_id).map_err(|e| MapperError::InvalidKsuid {
            id: new_order.customer_id.clone(),
            source: e,
        })?;

    // Handle shipping address: if override exists use it, otherwise use billing address
    let shipping_address = match &new_order.shipping_address_override {
        Some(override_address) => map_address_to_domain(override_address.clone()),
        None => map_address_to_domain(new_order.billing_address.clone()),
    };

    Ok(dmodels::OrderDomain {
        id: Ksuid::new(None, None),
        book_id,
        customer_id,
        quantity: new_order.quantity,
        shipping_date: new_order.shipping_date,
        billing_address: map_address_to_domain(new_order.billing_address),
        shipping_address,
        status: dmodels::OrderStatus::Placed,
    })
}

fn map_address_to_domain(address: rmodels::Address) -> dmodels::AddressDomain {
    dmodels::AddressDomain {
        street: address.street,
        street_number: address.street_number,
        zip_code: address.zip_code,
        city: address.city,
        province: address.province,
        country: address.country,
    }
}

fn map_inventory_to_rest(inventory: dmodels::InventoryDomain) -> rmodels::Inventory {
    rmodels::Inventory {
        books_available: inventory.books_available,
        books_reordered: inventory.books_reordered,
        books_out_of_stock: inventory.books_out_of_stock,
    }
}

fn map_author_to_rest(author: dmodels::AuthorDomain) -> rmodels::Author {
    rmodels::Author {
        id: author.id.to_string(),
        title: author.title,
        first_name: author.first_name,
        second_names: author.second_names,
        last_name: author.last_name,
        date_of_birth: author.date_of_birth,
        date_of_death: author.date_of_death,
    }
}

fn map_new_author_to_domain(new_author: rmodels::Author) -> dmodels::AuthorDomain {
    dmodels::AuthorDomain {
        id: Ksuid::new(None, None),
        title: new_author.title,
        first_name: new_author.first_name,
        last_name: new_author.last_name,
        second_names: new_author.second_names,
        date_of_birth: new_author.date_of_birth,
        date_of_death: new_author.date_of_death,
    }
}

fn map_new_genre_to_domain(genre: String) -> dmodels::GenereDomain {
    dmodels::GenereDomain {
        id: Ksuid::new(None, None),
        name: genre,
    }
}

fn map_genre_to_rest(genre: dmodels::GenereDomain) -> rmodels::Genre {
    rmodels::Genre {
        id: genre.id.to_string(),
        name: genre.name,
    }
}

fn map_new_discount_code_to_domain(
    new_discount: rmodels::NewDiscountCode,
) -> dmodels::DiscountCodeDomain {
    dmodels::DiscountCodeDomain {
        id: Ksuid::new(None, None),
        percentage_discount: new_discount.percentage_discount,
        valid_from: new_discount.valid_from,
        valid_to: new_discount.valid_to,
        code: new_discount.code,
    }
}

fn map_discount_code_to_rest(discount: dmodels::DiscountCodeDomain) -> rmodels::DiscountCode {
    rmodels::DiscountCode {
        id: discount.id.to_string(),
        percentage_discount: discount.percentage_discount,
        valid_from: discount.valid_from,
        valid_to: discount.valid_to,
        code: discount.code,
    }
}

fn map_new_book_to_domain(
    new_book: rmodels::NewBook,
) -> Result<dmodels::NewBookDomain, MapperError> {
    if new_book.available < 0 {
        return Err(MapperError::InvalidBooksAvailable {
            books_available: new_book.available.clone(),
            source: Box::new(BookAvailabilityError(new_book.available.clone())),
        });
    }
    let first_release = match new_book.first_release {
        Some(release) => release,
        None => new_book.release,
    };

    let edition = match new_book.edition {
        Some(e) => e,
        None => 1,
    };

    // map authorIds to Ksuid
    let d_authors = new_book
        .authors
        .into_iter()
        .map(|a| {
            Ksuid::from_str(&a).map_err(|e| MapperError::InvalidKsuid {
                id: a.clone(),
                source: e,
            })
        })
        .collect::<Result<Vec<Ksuid>, MapperError>>()?;

    // map genereIds to Ksuid
    let d_genres = match new_book.genres {
        Some(genres) => {
            let result: Result<Vec<Ksuid>, MapperError> = genres
                .into_iter()
                .map(|g| {
                    Ksuid::from_str(&g).map_err(|e| MapperError::InvalidKsuid {
                        id: g.clone(),
                        source: e,
                    })
                })
                .collect();
            Some(result?)
        }
        None => None,
    };

    // map discount codes to Ksuid
    let d_discounts = match new_book.discount_codes {
        Some(discounts) => {
            let result: Result<Vec<Ksuid>, MapperError> = discounts
                .into_iter()
                .map(|d| {
                    Ksuid::from_str(&d).map_err(|e| MapperError::InvalidKsuid {
                        id: d.clone(),
                        source: e,
                    })
                })
                .collect();
            Some(result?)
        }
        None => None,
    };

    Ok(dmodels::NewBookDomain {
        id: Ksuid::new(None, None),
        title: new_book.title,
        release: new_book.release,
        first_release,
        authors: d_authors,
        series: new_book.series,
        genres: d_genres,
        edition,
        price: new_book.price,
        discounts: d_discounts,
        available: new_book.available,
        status: dmodels::BookStatus::Available,
    })
}

fn map_book_to_rest(book: dmodels::BookDomain) -> rmodels::Book {
    // map the authors to the rest model
    let authors = book
        .authors
        .into_iter()
        .map(|a| map_author_to_rest(a))
        .collect();

    // math the genres to the rest model
    let genres = match book.genres {
        Some(genres) => {
            let result = genres.into_iter().map(|g| map_genre_to_rest(g)).collect();
            Some(result)
        }
        None => None,
    };

    // map the discount codes to the rest model
    let discounts = match book.discounts {
        Some(discounts) => {
            let result = discounts
                .into_iter()
                .map(|d| map_discount_code_to_rest(d))
                .collect();
            Some(result)
        }
        None => None,
    };

    rmodels::Book {
        id: book.id.to_string(),
        title: book.title,
        release: book.release,
        first_release: book.firs_release,
        series: book.series,
        authors,
        edition: book.edition,
        genres,
        discounts,
        price: book.price,
        available: book.available,
        status: book.status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_map_book_to_rest_with_all_fields() {
        // Arrange
        let book_id = Ksuid::new(None, None);
        let author_id = Ksuid::new(None, None);
        let genre_id = Ksuid::new(None, None);
        let discount_id = Ksuid::new(None, None);

        let author = dmodels::AuthorDomain {
            id: author_id,
            title: Some(String::from("Dr.")),
            first_name: String::from("John"),
            second_names: Some(vec![String::from("Middle")]),
            last_name: String::from("Doe"),
            date_of_birth: NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
            date_of_death: None,
        };

        let genre = dmodels::GenereDomain {
            id: genre_id,
            name: String::from("Fiction"),
        };

        let discount = dmodels::DiscountCodeDomain {
            id: discount_id,
            percentage_discount: 10,
            valid_from: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            valid_to: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
            code: String::from("DISCOUNT10"),
        };

        let book = dmodels::BookDomain {
            id: book_id,
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            firs_release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            authors: vec![author],
            series: Some(String::from("Test Series")),
            genres: Some(vec![genre]),
            edition: 1,
            price: 29.99,
            discounts: Some(vec![discount]),
            available: 10,
            status: dmodels::BookStatus::Available,
        };

        // Act
        let result = map_book_to_rest(book);

        // Assert
        assert_eq!(result.title, "Test Book");
        assert_eq!(result.authors.len(), 1);
        assert_eq!(result.authors[0].first_name, "John");
        assert!(result.genres.is_some());
        assert_eq!(result.genres.unwrap()[0].name, "Fiction");
        assert!(result.discounts.is_some());
        assert_eq!(result.available, 10);
        assert_eq!(result.status, "available");
    }

    #[test]
    fn test_map_book_to_rest_without_optional_fields() {
        // Arrange
        let book_id = Ksuid::new(None, None);
        let author_id = Ksuid::new(None, None);

        let author = dmodels::AuthorDomain {
            id: author_id,
            title: None,
            first_name: String::from("John"),
            second_names: None,
            last_name: String::from("Doe"),
            date_of_birth: NaiveDate::from_ymd_opt(2024, 12, 09).unwrap(),
            date_of_death: None,
        };

        let book = dmodels::BookDomain {
            id: book_id,
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            firs_release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            authors: vec![author],
            series: None,
            genres: None,
            edition: 1,
            price: 29.99,
            discounts: None,
            available: 10,
            status: dmodels::BookStatus::Available,
        };

        // Act
        let result = map_book_to_rest(book);

        // Assert
        assert_eq!(result.title, "Test Book");
        assert_eq!(result.authors.len(), 1);
        assert!(result.series.is_none());
        assert!(result.genres.is_none());
        assert!(result.discounts.is_none());
        assert_eq!(result.available, 10);
        assert_eq!(result.status, "available");
    }
}
