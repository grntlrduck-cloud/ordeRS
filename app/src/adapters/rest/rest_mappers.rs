use crate::domain::models as dmodels;
use openapi::models as rmodels;

pub fn map_address_to_rest(address: dmodels::AddressDomain) -> rmodels::Address {
    rmodels::Address {
        street: address.street,
        street_number: address.street_number,
        zip_code: address.zip_code,
        city: address.city,
        province: address.province,
        country: address.country,
    }
}

pub fn map_author_to_rest(author: dmodels::AuthorDomain) -> rmodels::Author {
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

pub fn map_book_to_rest(book: dmodels::BookDomain) -> rmodels::Book {
    // map the authors to the rest model
    let authors = book.authors.into_iter().map(map_author_to_rest).collect();

    // math the genres to the rest model
    let genres = match book.genres {
        Some(genres) => {
            let result = genres.into_iter().map(map_genre_to_rest).collect();
            Some(result)
        }
        None => None,
    };

    // map the discount codes to the rest model
    let discounts = match book.discounts {
        Some(discounts) => {
            let result = discounts
                .into_iter()
                .map(map_discount_code_to_rest)
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

pub fn map_discount_code_to_rest(discount: dmodels::DiscountCodeDomain) -> rmodels::DiscountCode {
    rmodels::DiscountCode {
        id: discount.id.to_string(),
        percentage_discount: discount.percentage_discount as u8,
        valid_from: discount.valid_from,
        valid_to: discount.valid_to,
        code: discount.code,
    }
}

pub fn map_genre_to_rest(genre: dmodels::GenereDomain) -> rmodels::Genre {
    rmodels::Genre {
        id: genre.id.to_string(),
        name: genre.name,
    }
}

pub fn map_inventory_to_rest(inventory: dmodels::InventoryDomain) -> rmodels::Inventory {
    rmodels::Inventory {
        books_available: inventory.books_available,
        books_reordered: inventory.books_reordered,
        books_out_of_stock: inventory.books_out_of_stock,
    }
}

pub fn map_order_to_rest(order: dmodels::OrderDomain) -> rmodels::Order {
    let address_override = if order.billing_address == order.shipping_address {
        None
    } else {
        Some(map_address_to_rest(order.shipping_address))
    };
    let books = order
        .books
        .into_iter()
        .map(|b| rmodels::OrderedBook {
            book_id: b.book_id.to_string(),
            quantity: b.quantity,
        })
        .collect();

    rmodels::Order {
        id: order.id.to_string(),
        customer_id: order.customer_id.to_string(),
        books,
        shipping_date: order.shipping_date,
        billing_address: map_address_to_rest(order.billing_address.clone()),
        shipping_address_override: address_override,
        status: order.status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models as dmodels;
    use chrono::{NaiveDate, Utc};
    use svix_ksuid::*;

    #[test]
    fn test_map_order_to_rest() {
        // Arrange
        let book_id = Ksuid::new(None, None);
        let order = dmodels::OrderDomain {
            id: Ksuid::new(None, None),
            customer_id: Ksuid::new(None, None),
            books: vec![dmodels::OrderedBookDomain {
                book_id,
                quantity: 2,
            }],
            shipping_date: Utc::now().date_naive(),
            billing_address: dmodels::AddressDomain {
                street: String::from("Main St"),
                street_number: String::from("123"),
                zip_code: String::from("12345"),
                city: String::from("City"),
                province: Some(String::from("Province")),
                country: String::from("Country"),
            },
            shipping_address: dmodels::AddressDomain {
                street: String::from("Second St"),
                street_number: String::from("456"),
                zip_code: String::from("67890"),
                city: String::from("Other City"),
                province: Some(String::from("Other Province")),
                country: String::from("Other Country"),
            },
            status: dmodels::OrderStatus::Placed,
        };

        // Act
        let result = map_order_to_rest(order);

        // Assert
        assert!(result.shipping_address_override.is_some());
        assert_eq!(result.billing_address.street, "Main St");
        assert_eq!(
            result.shipping_address_override.unwrap().street,
            "Second St"
        );
        assert_eq!(result.books.len(), 1);
        assert_eq!(result.books[0].quantity, 2);
        assert_eq!(result.books[0].book_id, book_id.to_string());
        assert_eq!(result.status, "placed");
    }

    #[test]
    fn test_map_order_to_rest_same_addresses() {
        // Arrange
        let billing_address = dmodels::AddressDomain {
            street: String::from("Main St"),
            street_number: String::from("123"),
            zip_code: String::from("12345"),
            city: String::from("City"),
            province: Some(String::from("Province")),
            country: String::from("Country"),
        };

        let order = dmodels::OrderDomain {
            id: Ksuid::new(None, None),
            books: vec![dmodels::OrderedBookDomain {
                book_id: Ksuid::new(None, None),
                quantity: 2,
            }],
            customer_id: Ksuid::new(None, None),
            shipping_date: Utc::now().date_naive(),
            billing_address: billing_address.clone(),
            shipping_address: billing_address,
            status: dmodels::OrderStatus::Placed,
        };

        // Act
        let result = map_order_to_rest(order);

        // Assert
        assert!(result.shipping_address_override.is_none());
        assert_eq!(result.billing_address.street, "Main St");
    }

    #[test]
    fn test_map_inventory_to_rest() {
        // Arrange
        let inventory = dmodels::InventoryDomain {
            books_available: 10,
            books_reordered: 5,
            books_out_of_stock: 2,
        };

        // Act
        let result = map_inventory_to_rest(inventory);

        // Assert
        assert_eq!(result.books_available, 10);
        assert_eq!(result.books_reordered, 5);
        assert_eq!(result.books_out_of_stock, 2);
    }

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
            date_of_birth: NaiveDate::from_ymd_opt(2024, 12, 9).unwrap(),
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
