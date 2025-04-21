use std::str::FromStr;

use super::mapper_errors::*;
use crate::domain::models as dmodels;
use openapi::models as rmodels;
use svix_ksuid::*;

pub fn map_address_to_domain(address: &rmodels::Address) -> dmodels::AddressDomain {
    dmodels::AddressDomain {
        street: address.street.clone(),
        street_number: address.street_number.clone(),
        zip_code: address.zip_code.clone(),
        city: address.city.clone(),
        province: address.province.clone(),
        country: address.country.clone(),
    }
}

pub fn map_author_update_props_to_domain(
    id: &str,
    props: &rmodels::AuthorProperties,
) -> Result<dmodels::AuthorUpdateProps, MapperError> {
    let kid = Ksuid::from_str(&id).map_err(|e| MapperError::InvalidKsuid {
        id: String::from(id),
        source: e,
    })?;
    Ok(dmodels::AuthorUpdateProps {
        id: kid,
        date_of_death: props.date_of_death.clone(),
        last_name: props.last_name.clone(),
        second_names: props.second_names.clone(),
        title: props.title.clone(),
    })
}

pub fn map_book_props_to_domain(
    id: &str,
    props: &rmodels::BookProperties,
) -> Result<dmodels::BookUpdateProps, MapperError> {
    let kid = Ksuid::from_str(id).map_err(|e| MapperError::InvalidKsuid {
        id: String::from(id),
        source: e,
    })?;

    let authors = match &props.authors {
        Some(authors) => {
            let result = map_strings_to_ksuids(authors);
            Some(result?)
        }
        None => None,
    };

    let genres = match &props.genres {
        Some(genres) => {
            let result = map_strings_to_ksuids(genres);
            Some(result?)
        }
        None => None,
    };

    let discounts = match &props.discount_codes {
        Some(discounts) => {
            let result = map_strings_to_ksuids(discounts);
            Some(result?)
        }
        None => None,
    };

    let status = match &props.status {
        Some(status) => {
            let result = dmodels::BookStatus::from_str(&status).map_err(|_| {
                MapperError::InvalidBookStatus {
                    status: status.clone(),
                    source: Box::new(BookStatusError(status.clone())),
                }
            })?;
            Some(result)
        }
        None => None,
    };

    Ok(dmodels::BookUpdateProps {
        id: kid,
        authors,
        available: props.available,
        discounts,
        genres,
        edition: props.edition,
        price: props.price,
        release: props.release,
        series: props.series.clone(),
        status,
        title: props.title.clone(),
    })
}

pub fn map_new_author_to_domain(new_author: &rmodels::NewAuthor) -> dmodels::AuthorDomain {
    dmodels::AuthorDomain {
        id: Ksuid::new(None, None),
        title: new_author.title.clone(),
        first_name: new_author.first_name.clone(),
        last_name: new_author.last_name.clone(),
        second_names: new_author.second_names.clone(),
        date_of_birth: new_author.date_of_birth.clone(),
        date_of_death: new_author.date_of_death.clone(),
    }
}

pub fn map_new_book_to_domain(
    new_book: &rmodels::NewBook,
) -> Result<dmodels::NewBookDomain, MapperError> {
    if new_book.available < 0 {
        return Err(MapperError::BooksAvailableOutOfBound {
            books_available: new_book.available,
            source: Box::new(BookAvailabilityError(new_book.available)),
        });
    }
    let first_release = match new_book.first_release {
        Some(release) => release,
        None => new_book.release,
    };

    let edition = new_book.edition.unwrap_or(1);

    // map authorIds to Ksuid
    let d_authors = map_strings_to_ksuids(&new_book.authors)?;

    // map genereIds to Ksuid
    let d_genres = match &new_book.genres {
        Some(genres) => {
            let result = map_strings_to_ksuids(genres);
            Some(result?)
        }
        None => None,
    };

    // map discount codes to Ksuid
    let d_discounts = match &new_book.discount_codes {
        Some(discounts) => {
            let result = map_strings_to_ksuids(&discounts);
            Some(result?)
        }
        None => None,
    };

    Ok(dmodels::NewBookDomain {
        id: Ksuid::new(None, None),
        title: new_book.title.clone(),
        release: new_book.release,
        first_release,
        authors: d_authors,
        series: new_book.series.clone(),
        genres: d_genres,
        edition,
        price: new_book.price,
        discounts: d_discounts.clone(),
        available: new_book.available,
        status: dmodels::BookStatus::Available,
    })
}

pub fn map_new_discount_code_to_domain(
    new_discount: &rmodels::NewDiscountCode,
) -> Result<dmodels::DiscountCodeDomain, MapperError> {
    if new_discount.percentage_discount < 1 || new_discount.percentage_discount > 80 {
        return Err(MapperError::DiscountPercentageOutOfBounds {
            percentage: new_discount.percentage_discount,
            source: Box::new(DiscountPercentageError(new_discount.percentage_discount)),
        });
    }

    Ok(dmodels::DiscountCodeDomain {
        id: Ksuid::new(None, None),
        percentage_discount: new_discount.percentage_discount,
        valid_from: new_discount.valid_from,
        valid_to: new_discount.valid_to,
        code: new_discount.code.clone(),
    })
}

pub fn map_new_genre_to_domain(genre: &str) -> dmodels::GenereDomain {
    dmodels::GenereDomain {
        id: Ksuid::new(None, None),
        name: String::from(genre),
    }
}

pub fn map_new_order_to_domain(
    new_order: &rmodels::NewOrder,
) -> Result<dmodels::OrderDomain, MapperError> {
    let books = Vec::to_owned(&new_order.books)
        .into_iter()
        .map(|b| {
            if b.quantity < 1 {
                return Err(MapperError::OrderQuantityOutOfBounds {
                    quantity: b.quantity,
                    source: Box::new(OrderQuantityError(b.quantity)),
                });
            }
            let book_id = Ksuid::from_str(&b.book_id).map_err(|e| MapperError::InvalidKsuid {
                id: b.book_id.clone(),
                source: e,
            })?;
            Ok(dmodels::OrderedBookDomain {
                book_id,
                quantity: b.quantity,
            })
        })
        .collect::<Result<Vec<dmodels::OrderedBookDomain>, MapperError>>()?;

    let customer_id =
        Ksuid::from_str(&new_order.customer_id).map_err(|e| MapperError::InvalidKsuid {
            id: new_order.customer_id.clone(),
            source: e,
        })?;

    // Handle shipping address: if override exists use it, otherwise use billing address
    let shipping_address = match &new_order.shipping_address_override {
        Some(override_address) => map_address_to_domain(&override_address.clone()),
        None => map_address_to_domain(&new_order.billing_address.clone()),
    };

    Ok(dmodels::OrderDomain {
        id: Ksuid::new(None, None),
        books,
        customer_id,
        shipping_date: new_order.shipping_date,
        billing_address: map_address_to_domain(&new_order.billing_address),
        shipping_address,
        status: dmodels::OrderStatus::Placed,
    })
}

pub fn map_order_props_to_domain(
    id: &str,
    props: &rmodels::OrderProperties,
) -> Result<dmodels::OrderUpdateProps, MapperError> {
    let kid = Ksuid::from_str(id).map_err(|e| MapperError::InvalidKsuid {
        id: String::from(id),
        source: e,
    })?;

    let status = dmodels::OrderStatus::from_str(&props.status).map_err(|_| {
        MapperError::InvalidOrderStatus {
            status: props.status.clone(),
            source: Box::new(OrderStatusError(props.status.clone())),
        }
    })?;

    Ok(dmodels::OrderUpdateProps {
        id: kid,
        shipping_date: props.shipping_date,
        status,
    })
}

pub fn map_strings_to_ksuids(ids_str: &Vec<String>) -> Result<Vec<Ksuid>, MapperError> {
    ids_str
        .into_iter()
        .map(|id| {
            Ksuid::from_str(&id).map_err(|e| MapperError::InvalidKsuid {
                id: id.clone(),
                source: e,
            })
        })
        .collect::<Result<Vec<Ksuid>, MapperError>>()
}

pub fn map_book_status_list_to_domain(
    status_str: &Vec<String>,
) -> Result<Vec<dmodels::BookStatus>, MapperError> {
    status_str
        .into_iter()
        .map(|s| {
            dmodels::BookStatus::from_str(&s).map_err(|_| MapperError::InvalidBookStatus {
                status: s.clone(),
                source: Box::new(BookStatusError(s.clone())),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::BookStatus;
    use chrono::{DateTime, NaiveDate, Utc};
    use openapi::models as rmodels;

    #[test]
    fn test_map_new_order_to_domain_success() {
        // Arrange
        let new_order = rmodels::NewOrder {
            customer_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
            books: vec![rmodels::OrderedBook {
                book_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
                quantity: 2,
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: rmodels::Address {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address_override: None,
        };

        // Act
        let result = map_new_order_to_domain(&new_order);

        // Assert
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.books.len(), 1);
        assert_eq!(order.books[0].quantity, 2);
        assert_eq!(order.status, dmodels::OrderStatus::Placed);
        assert_eq!(order.billing_address.street, "Main St");
        assert_eq!(order.shipping_address.street, "Main St"); // Same as billing since no override
    }

    #[test]
    fn test_map_new_order_to_domain_with_shipping_override() {
        // Arrange
        let new_order = rmodels::NewOrder {
            customer_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
            books: vec![rmodels::OrderedBook {
                book_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
                quantity: 2,
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: rmodels::Address {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address_override: Some(rmodels::Address {
                street: String::from("Second St"),
                street_number: String::from("456"),
                zip_code: String::from("67890"),
                city: String::from("Other City"),
                province: Some(String::from("Other Province")),
                country: String::from("Other Country"),
            }),
        };

        // Act
        let result = map_new_order_to_domain(&new_order);

        // Assert
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.billing_address.street, "Main St");
        assert_eq!(order.shipping_address.street, "Second St");
    }

    #[test]
    fn test_map_new_order_to_domain_invalid_book_id() {
        // Arrange
        let new_order = rmodels::NewOrder {
            customer_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
            books: vec![rmodels::OrderedBook {
                book_id: String::from("invalid-id"),
                quantity: 2,
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: rmodels::Address {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address_override: None,
        };

        // Act
        let result = map_new_order_to_domain(&new_order);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_new_order_to_domain_invalid_customer_id() {
        // Arrange
        let new_order = rmodels::NewOrder {
            customer_id: String::from("invalid-id"),
            books: vec![rmodels::OrderedBook {
                book_id: String::from("invalid-id"),
                quantity: 2,
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: rmodels::Address {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address_override: None,
        };

        // Act
        let result = map_new_order_to_domain(&new_order);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_new_order_to_domain_invalid_quantity() {
        // Arrange
        let new_order = rmodels::NewOrder {
            customer_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
            books: vec![rmodels::OrderedBook {
                book_id: String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3"),
                quantity: 0, // Invalid quantity - less than 1
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: rmodels::Address {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address_override: None,
        };

        // Act
        let result = map_new_order_to_domain(&new_order);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::OrderQuantityOutOfBounds { quantity, .. }) => {
                assert_eq!(quantity, 0);
            }
            _ => panic!("Expected OrderQuantityOutOfBounds error"),
        }
    }

    #[test]
    fn test_map_new_book_to_domain_success() {
        // Arrange
        let new_book = rmodels::NewBook {
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            first_release: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            authors: vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")],
            series: Some(String::from("Test Series")),
            genres: Some(vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")]),
            edition: Some(1),
            price: 29.99,
            discount_codes: Some(vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")]),
            available: 10,
        };

        // Act
        let result = map_new_book_to_domain(&new_book);

        // Assert
        assert!(result.is_ok());
        let book = result.unwrap();
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.available, 10);
        assert_eq!(book.status, dmodels::BookStatus::Available);
        assert_eq!(book.authors.len(), 1);
        assert!(book.genres.is_some());
        assert!(book.discounts.is_some());
    }

    #[test]
    fn test_map_new_book_to_domain_with_negative_available() {
        // Arrange
        let new_book = rmodels::NewBook {
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            first_release: None,
            authors: vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")],
            series: None,
            genres: None,
            edition: None,
            price: 29.99,
            discount_codes: None,
            available: -1,
        };

        // Act
        let result = map_new_book_to_domain(&new_book);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::BooksAvailableOutOfBound {
                books_available, ..
            }) => {
                assert_eq!(books_available, -1);
            }
            _ => panic!("Expected BooksAvailableOutOfBound error"),
        }
    }

    #[test]
    fn test_map_new_book_to_domain_with_invalid_author_id() {
        // Arrange
        let new_book = rmodels::NewBook {
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            first_release: None,
            authors: vec![String::from("invalid-ksuid")],
            series: None,
            genres: None,
            edition: None,
            price: 29.99,
            discount_codes: None,
            available: 10,
        };

        // Act
        let result = map_new_book_to_domain(&new_book);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-ksuid");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_new_book_to_domain_with_invalid_genre_id() {
        // Arrange
        let new_book = rmodels::NewBook {
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            first_release: None,
            authors: vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")],
            series: None,
            genres: Some(vec![String::from("invalid-ksuid")]),
            edition: None,
            price: 29.99,
            discount_codes: None,
            available: 10,
        };

        // Act
        let result = map_new_book_to_domain(&new_book);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-ksuid");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_new_book_to_domain_with_invalid_discount_id() {
        // Arrange
        let new_book = rmodels::NewBook {
            title: String::from("Test Book"),
            release: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            first_release: None,
            authors: vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")],
            series: None,
            genres: None,
            edition: None,
            price: 29.99,
            discount_codes: Some(vec![String::from("invalid-ksuid")]),
            available: 10,
        };

        // Act
        let result = map_new_book_to_domain(&new_book);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-ksuid");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_new_author_to_domain_with_all_fields() {
        // Arrange
        let new_author = rmodels::NewAuthor {
            title: Some(String::from("Dr.")),
            first_name: String::from("John"),
            second_names: Some(vec![String::from("Middle"), String::from("Name")]),
            last_name: String::from("Doe"),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            date_of_death: Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        };

        // Act
        let result = map_new_author_to_domain(&new_author);

        // Assert
        assert_eq!(result.first_name, "John");
        assert_eq!(result.last_name, "Doe");
        assert_eq!(result.title.unwrap(), "Dr.");
        assert_eq!(result.second_names.unwrap().len(), 2);
        assert_eq!(
            result.date_of_birth,
            NaiveDate::from_ymd_opt(1980, 1, 1).unwrap()
        );
        assert!(result.date_of_death.is_some());
    }

    #[test]
    fn test_map_new_author_to_domain_minimal_fields() {
        // Arrange
        let new_author = rmodels::NewAuthor {
            title: None,
            first_name: String::from("John"),
            second_names: None,
            last_name: String::from("Doe"),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            date_of_death: None,
        };

        // Act
        let result = map_new_author_to_domain(&new_author);

        // Assert
        assert_eq!(result.first_name, "John");
        assert_eq!(result.last_name, "Doe");
        assert!(result.title.is_none());
        assert!(result.second_names.is_none());
        assert!(result.date_of_death.is_none());
    }

    #[test]
    fn test_map_new_genre_to_domain() {
        // Arrange
        let genre_name = "Science Fiction";

        // Act
        let result = map_new_genre_to_domain(genre_name);

        // Assert
        assert_eq!(result.name, genre_name);
        assert!(!result.id.to_string().is_empty()); // Verify ID was generated
    }

    #[test]
    fn test_map_new_genre_to_domain_empty_name() {
        let result = map_new_genre_to_domain("");

        // Assert
        assert_eq!(result.name, "");
        assert!(!result.id.to_string().is_empty());
    }

    #[test]
    fn test_map_new_discount_code_to_domain() {
        // Arrange
        let new_discount = rmodels::NewDiscountCode {
            percentage_discount: 25,
            valid_from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_to: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            code: String::from("SAVE25"),
        };

        // Act
        let result = map_new_discount_code_to_domain(&new_discount);

        // Assert
        assert!(result.is_ok());
        let discount = result.unwrap();
        assert_eq!(discount.percentage_discount, 25);
        assert_eq!(discount.code, "SAVE25");
        assert_eq!(
            discount.valid_from,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            discount.valid_to,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
        assert!(!discount.id.to_string().is_empty());
    }

    #[test]
    fn test_map_new_discount_code_to_domain_invalid_percentage_large() {
        // Arrange
        let new_discount = rmodels::NewDiscountCode {
            percentage_discount: 90,
            valid_from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_to: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            code: String::from("INVALID90"),
        };

        // Act
        let result = map_new_discount_code_to_domain(&new_discount);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::DiscountPercentageOutOfBounds { percentage, .. }) => {
                assert_eq!(percentage, 90);
            }
            _ => panic!("Expected DiscountPercentageOutOfBounds error"),
        }
    }

    #[test]
    fn test_map_new_discount_code_to_domain_invalid_percentage_small() {
        // Arrange
        let new_discount = rmodels::NewDiscountCode {
            percentage_discount: 0,
            valid_from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_to: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            code: String::from("INVALID90"),
        };

        // Act
        let result = map_new_discount_code_to_domain(&new_discount);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::DiscountPercentageOutOfBounds { percentage, .. }) => {
                assert_eq!(percentage, 0);
            }
            _ => panic!("Expected DiscountPercentageOutOfBounds error"),
        }
    }

    #[test]
    fn test_map_new_discount_code_to_domain_edge_cases() {
        // Arrange
        let new_discount = rmodels::NewDiscountCode {
            percentage_discount: 80, // Maximum allowed discount
            valid_from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_to: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // Same day validity
            code: String::from(""),                                 // Empty code
        };

        // Act
        let result = map_new_discount_code_to_domain(&new_discount);

        // Assert
        assert!(result.is_ok());
        let discount = result.unwrap();
        assert_eq!(discount.percentage_discount, 80);
        assert_eq!(discount.code, "");
        assert_eq!(discount.valid_from, discount.valid_to); // Same day
        assert!(!discount.id.to_string().is_empty());
    }

    #[test]
    fn test_map_order_props_to_domain_success() {
        // Arrange
        let order_props = rmodels::OrderProperties {
            shipping_date: DateTime::from_timestamp(1640995200, 0).unwrap(),
            status: String::from("shipped"),
        };

        // Act
        let result = map_order_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &order_props);

        // Assert
        assert!(result.is_ok());
        let props = result.unwrap();
        assert_eq!(props.status, dmodels::OrderStatus::Shipped);
    }

    #[test]
    fn test_map_order_props_to_domain_invalid_id() {
        // Arrange
        let order_props = rmodels::OrderProperties {
            shipping_date: DateTime::from_timestamp(1640995200, 0).unwrap(),
            status: String::from("shipped"),
        };

        // Act
        let result = map_order_props_to_domain("invalid-id", &order_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_order_props_to_domain_invalid_status() {
        // Arrange
        let order_props = rmodels::OrderProperties {
            shipping_date: DateTime::from_timestamp(1640995200, 0).unwrap(),
            status: String::from("invalid-status"),
        };

        // Act
        let result = map_order_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &order_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidOrderStatus { status, .. }) => {
                assert_eq!(status, "invalid-status");
            }
            _ => panic!("Expected InvalidOrderStatus error"),
        }
    }

    #[test]
    fn test_map_book_props_to_domain_success() {
        // Arrange
        let book_props = rmodels::BookProperties {
            title: Some(String::from("Updated Title")),
            release: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            authors: Some(vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")]),
            series: Some(String::from("Updated Series")),
            genres: Some(vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")]),
            edition: Some(2),
            price: Some(39.99),
            discount_codes: Some(vec![String::from("2N1yQqzh1fhkGEPv5rJRqOZqxE3")]),
            available: Some(15),
            status: Some(String::from("available")),
        };

        // Act
        let result = map_book_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &book_props);

        // Assert
        assert!(result.is_ok());
        let props = result.unwrap();
        assert_eq!(props.title.unwrap(), "Updated Title");
        assert_eq!(props.available.unwrap(), 15);
        assert_eq!(props.status.unwrap(), dmodels::BookStatus::Available);
    }

    #[test]
    fn test_map_book_props_to_domain_minimal_fields() {
        // Arrange
        let book_props = rmodels::BookProperties {
            title: Some(String::from("Updated Title")),
            release: None,
            authors: None,
            series: None,
            genres: None,
            edition: None,
            price: None,
            discount_codes: None,
            available: None,
            status: None,
        };

        // Act
        let result = map_book_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &book_props);

        // Assert
        assert!(result.is_ok());
        let props = result.unwrap();
        assert_eq!(props.title.unwrap(), "Updated Title");
        assert!(props.release.is_none());
        assert!(props.authors.is_none());
        assert!(props.status.is_none());
    }

    #[test]
    fn test_map_book_props_to_domain_invalid_id() {
        // Arrange
        let book_props = rmodels::BookProperties {
            title: Some(String::from("Updated Title")),
            release: None,
            authors: None,
            series: None,
            genres: None,
            edition: None,
            price: None,
            discount_codes: None,
            available: None,
            status: None,
        };

        // Act
        let result = map_book_props_to_domain("invalid-id", &book_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_book_props_to_domain_invalid_author_id() {
        // Arrange
        let book_props = rmodels::BookProperties {
            title: Some(String::from("Updated Title")),
            release: None,
            authors: Some(vec![String::from("invalid-author-id")]),
            series: None,
            genres: None,
            edition: None,
            price: None,
            discount_codes: None,
            available: None,
            status: None,
        };

        // Act
        let result = map_book_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &book_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-author-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_book_props_to_domain_invalid_status() {
        // Arrange
        let book_props = rmodels::BookProperties {
            title: Some(String::from("Updated Title")),
            release: None,
            authors: None,
            series: None,
            genres: None,
            edition: None,
            price: None,
            discount_codes: None,
            available: None,
            status: Some(String::from("invalid-status")),
        };

        // Act
        let result = map_book_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &book_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidBookStatus { status, .. }) => {
                assert_eq!(status, "invalid-status");
            }
            _ => panic!("Expected InvalidBookStatus error"),
        }
    }

    #[test]
    fn test_map_author_update_props_to_domain_success() {
        // Arrange
        let author_props = rmodels::AuthorProperties {
            title: Some(String::from("Dr.")),
            last_name: Some(String::from("Updated")),
            second_names: Some(vec![String::from("Middle"), String::from("Name")]),
            date_of_death: Some(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
        };

        // Act
        let result =
            map_author_update_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &author_props);

        // Assert
        assert!(result.is_ok());
        let props = result.unwrap();
        assert_eq!(props.title.unwrap(), "Dr.");
        assert_eq!(props.last_name.unwrap(), "Updated");
        assert_eq!(props.second_names.unwrap().len(), 2);
    }

    #[test]
    fn test_map_author_update_props_to_domain_minimal_fields() {
        // Arrange
        let author_props = rmodels::AuthorProperties {
            title: None,
            last_name: Some(String::from("Updated")),
            second_names: None,
            date_of_death: None,
        };

        // Act
        let result =
            map_author_update_props_to_domain("2N1yQqzh1fhkGEPv5rJRqOZqxE3", &author_props);

        // Assert
        assert!(result.is_ok());
        let props = result.unwrap();
        assert!(props.title.is_none());
        assert_eq!(props.last_name.unwrap(), "Updated");
        assert!(props.second_names.is_none());
        assert!(props.date_of_death.is_none());
    }

    #[test]
    fn test_map_author_update_props_to_domain_invalid_id() {
        // Arrange
        let author_props = rmodels::AuthorProperties {
            title: None,
            last_name: Some(String::from("Updated")),
            second_names: None,
            date_of_death: None,
        };

        // Act
        let result = map_author_update_props_to_domain("invalid-id", &author_props);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidKsuid { id, .. }) => {
                assert_eq!(id, "invalid-id");
            }
            _ => panic!("Expected InvalidKsuid error"),
        }
    }

    #[test]
    fn test_map_book_status_list_to_domain_success() {
        // Arrange
        let status_list = vec![
            String::from("available"),
            String::from("out-of-stock"),
            String::from("re-ordered"),
        ];

        // Act
        let result = map_book_status_list_to_domain(&status_list);

        // Assert
        assert!(result.is_ok());
        let statuses = result.unwrap();
        assert_eq!(statuses.len(), 3);
        assert_eq!(statuses[0], BookStatus::Available);
        assert_eq!(statuses[1], BookStatus::OutOfStock);
        assert_eq!(statuses[2], BookStatus::ReOrdered);
    }

    #[test]
    fn test_map_book_status_list_to_domain_invalid_status() {
        // Arrange
        let status_list = vec![
            String::from("available"),
            String::from("invalid_status"),
            String::from("re-ordered"),
        ];

        // Act
        let result = map_book_status_list_to_domain(&status_list);

        // Assert
        assert!(result.is_err());
        match result {
            Err(MapperError::InvalidBookStatus { status, .. }) => {
                assert_eq!(status, "invalid_status");
            }
            _ => panic!("Expected InvalidBookStatus error"),
        }
    }

    #[test]
    fn test_map_book_status_list_to_domain_empty_list() {
        // Arrange
        let status_list: Vec<String> = vec![];

        // Act
        let result = map_book_status_list_to_domain(&status_list);

        // Assert
        assert!(result.is_ok());
        let statuses = result.unwrap();
        assert!(statuses.is_empty());
    }
}
