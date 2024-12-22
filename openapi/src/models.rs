#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteAuthorPathParams {
    /// author to delete
    pub author_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetAuthorByIdPathParams {
    /// Id of author to return
    pub author_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateAuthorPathParams {
    /// Id of author to return
    pub author_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteBookPathParams {
    /// BookId to delete
    pub book_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetBookByIdPathParams {
    /// Id of book to return
    pub book_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetBooksByAuthorsQueryParams {
    /// Authors to filter by
    #[serde(rename = "authors")]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetBooksByGeneresQueryParams {
    /// Generef to filter by
    #[serde(rename = "genres")]
    pub genres: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetBooksByStatusQueryParams {
    /// Status to filter by
    #[serde(rename = "status")]
    pub status: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateBookPathParams {
    /// Id of book to return
    pub book_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteDiscountPathParams {
    /// discount to delete
    pub discount_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetDiscountByIdPathParams {
    /// Id of discount to return
    pub discount_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateDiscountPathParams {
    /// Id of discount to return
    pub discount_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteOrderPathParams {
    /// ID of the order that needs to be deleted
    pub order_id: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetOrderByIdPathParams {
    /// Id of order that needs to be fetched
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateOrderPathParams {
    /// Id of order to return
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Address {
    #[serde(rename = "street")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    #[serde(rename = "street_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_number: Option<String>,

    #[serde(rename = "zip_code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,

    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(rename = "province")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,

    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl Address {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Address {
        Address {
            street: None,
            street_number: None,
            zip_code: None,
            city: None,
            province: None,
            country: None,
        }
    }
}

/// Converts the Address value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.street
                .as_ref()
                .map(|street| ["street".to_string(), street.to_string()].join(",")),
            self.street_number.as_ref().map(|street_number| {
                ["street_number".to_string(), street_number.to_string()].join(",")
            }),
            self.zip_code
                .as_ref()
                .map(|zip_code| ["zip_code".to_string(), zip_code.to_string()].join(",")),
            self.city
                .as_ref()
                .map(|city| ["city".to_string(), city.to_string()].join(",")),
            self.province
                .as_ref()
                .map(|province| ["province".to_string(), province.to_string()].join(",")),
            self.country
                .as_ref()
                .map(|country| ["country".to_string(), country.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Address value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Address {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub street: Vec<String>,
            pub street_number: Vec<String>,
            pub zip_code: Vec<String>,
            pub city: Vec<String>,
            pub province: Vec<String>,
            pub country: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Address".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "street" => intermediate_rep.street.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "street_number" => intermediate_rep.street_number.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "zip_code" => intermediate_rep.zip_code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "city" => intermediate_rep.city.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "province" => intermediate_rep.province.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "country" => intermediate_rep.country.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Address".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Address {
            street: intermediate_rep.street.into_iter().next(),
            street_number: intermediate_rep.street_number.into_iter().next(),
            zip_code: intermediate_rep.zip_code.into_iter().next(),
            city: intermediate_rep.city.into_iter().next(),
            province: intermediate_rep.province.into_iter().next(),
            country: intermediate_rep.country.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Address> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Address>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Address>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Address - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Address> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Address as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Address - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Author {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "second_names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_names: Option<Vec<String>>,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "date_of_birth")]
    pub date_of_birth: chrono::naive::NaiveDate,

    #[serde(rename = "date_of_death")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<chrono::naive::NaiveDate>,
}

impl Author {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        first_name: String,
        last_name: String,
        date_of_birth: chrono::naive::NaiveDate,
    ) -> Author {
        Author {
            id,
            title: None,
            first_name,
            second_names: None,
            last_name,
            date_of_birth,
            date_of_death: None,
        }
    }
}

/// Converts the Author value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            self.title
                .as_ref()
                .map(|title| ["title".to_string(), title.to_string()].join(",")),
            Some("first_name".to_string()),
            Some(self.first_name.to_string()),
            self.second_names.as_ref().map(|second_names| {
                [
                    "second_names".to_string(),
                    second_names
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            Some("last_name".to_string()),
            Some(self.last_name.to_string()),
            // Skipping date_of_birth in query parameter serialization

            // Skipping date_of_death in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Author value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Author {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub title: Vec<String>,
            pub first_name: Vec<String>,
            pub second_names: Vec<Vec<String>>,
            pub last_name: Vec<String>,
            pub date_of_birth: Vec<chrono::naive::NaiveDate>,
            pub date_of_death: Vec<chrono::naive::NaiveDate>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Author".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_name" => intermediate_rep.first_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "second_names" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Author"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "last_name" => intermediate_rep.last_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "date_of_birth" => intermediate_rep.date_of_birth.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "date_of_death" => intermediate_rep.date_of_death.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Author".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Author {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in Author".to_string())?,
            title: intermediate_rep.title.into_iter().next(),
            first_name: intermediate_rep
                .first_name
                .into_iter()
                .next()
                .ok_or_else(|| "first_name missing in Author".to_string())?,
            second_names: intermediate_rep.second_names.into_iter().next(),
            last_name: intermediate_rep
                .last_name
                .into_iter()
                .next()
                .ok_or_else(|| "last_name missing in Author".to_string())?,
            date_of_birth: intermediate_rep
                .date_of_birth
                .into_iter()
                .next()
                .ok_or_else(|| "date_of_birth missing in Author".to_string())?,
            date_of_death: intermediate_rep.date_of_death.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Author> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Author>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Author>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Author - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Author> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Author as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Author - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthorProperties {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "first_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "second_names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_names: Option<Vec<String>>,

    #[serde(rename = "last_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(rename = "date_of_death")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<chrono::naive::NaiveDate>,
}

impl AuthorProperties {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> AuthorProperties {
        AuthorProperties {
            title: None,
            first_name: None,
            second_names: None,
            last_name: None,
            date_of_death: None,
        }
    }
}

/// Converts the AuthorProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AuthorProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.title
                .as_ref()
                .map(|title| ["title".to_string(), title.to_string()].join(",")),
            self.first_name
                .as_ref()
                .map(|first_name| ["first_name".to_string(), first_name.to_string()].join(",")),
            self.second_names.as_ref().map(|second_names| {
                [
                    "second_names".to_string(),
                    second_names
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.last_name
                .as_ref()
                .map(|last_name| ["last_name".to_string(), last_name.to_string()].join(",")),
            // Skipping date_of_death in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthorProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthorProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub first_name: Vec<String>,
            pub second_names: Vec<Vec<String>>,
            pub last_name: Vec<String>,
            pub date_of_death: Vec<chrono::naive::NaiveDate>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AuthorProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_name" => intermediate_rep.first_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "second_names" => return std::result::Result::Err(
                        "Parsing a container in this style is not supported in AuthorProperties"
                            .to_string(),
                    ),
                    #[allow(clippy::redundant_clone)]
                    "last_name" => intermediate_rep.last_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "date_of_death" => intermediate_rep.date_of_death.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AuthorProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthorProperties {
            title: intermediate_rep.title.into_iter().next(),
            first_name: intermediate_rep.first_name.into_iter().next(),
            second_names: intermediate_rep.second_names.into_iter().next(),
            last_name: intermediate_rep.last_name.into_iter().next(),
            date_of_death: intermediate_rep.date_of_death.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AuthorProperties> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthorProperties>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AuthorProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AuthorProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AuthorProperties> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AuthorProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AuthorProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Book {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "title")]
    pub title: String,

    /// the date when this edition of the book was released
    #[serde(rename = "release")]
    pub release: chrono::naive::NaiveDate,

    /// the date when the first edition of the book was release
    #[serde(rename = "first_release")]
    pub first_release: chrono::naive::NaiveDate,

    #[serde(rename = "authors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<models::Author>>,

    #[serde(rename = "generes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generes: Option<models::Genre>,

    #[serde(rename = "series")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// the edition of this book
    #[serde(rename = "edition")]
    pub edition: i32,

    /// the price of this book in Dollar
    #[serde(rename = "price")]
    pub price: f64,

    #[serde(rename = "discounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<models::DiscountCode>>,

    /// The number of available items
    #[serde(rename = "available")]
    pub available: i32,

    /// the inventory state of the book
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: String,
}

impl Book {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        title: String,
        release: chrono::naive::NaiveDate,
        first_release: chrono::naive::NaiveDate,
        edition: i32,
        price: f64,
        available: i32,
        status: String,
    ) -> Book {
        Book {
            id,
            title,
            release,
            first_release,
            authors: None,
            generes: None,
            series: None,
            edition,
            price,
            discounts: None,
            available,
            status,
        }
    }
}

/// Converts the Book value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("title".to_string()),
            Some(self.title.to_string()),
            // Skipping release in query parameter serialization

            // Skipping first_release in query parameter serialization

            // Skipping authors in query parameter serialization

            // Skipping generes in query parameter serialization
            self.series
                .as_ref()
                .map(|series| ["series".to_string(), series.to_string()].join(",")),
            Some("edition".to_string()),
            Some(self.edition.to_string()),
            Some("price".to_string()),
            Some(self.price.to_string()),
            // Skipping discounts in query parameter serialization
            Some("available".to_string()),
            Some(self.available.to_string()),
            Some("status".to_string()),
            Some(self.status.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Book value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Book {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub title: Vec<String>,
            pub release: Vec<chrono::naive::NaiveDate>,
            pub first_release: Vec<chrono::naive::NaiveDate>,
            pub authors: Vec<Vec<models::Author>>,
            pub generes: Vec<models::Genre>,
            pub series: Vec<String>,
            pub edition: Vec<i32>,
            pub price: Vec<f64>,
            pub discounts: Vec<Vec<models::DiscountCode>>,
            pub available: Vec<i32>,
            pub status: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err("Missing value while parsing Book".to_string())
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "release" => intermediate_rep.release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_release" => intermediate_rep.first_release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "authors" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Book"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "generes" => intermediate_rep.generes.push(
                        <models::Genre as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "series" => intermediate_rep.series.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "edition" => intermediate_rep.edition.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "discounts" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Book"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "available" => intermediate_rep.available.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Book".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Book {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in Book".to_string())?,
            title: intermediate_rep
                .title
                .into_iter()
                .next()
                .ok_or_else(|| "title missing in Book".to_string())?,
            release: intermediate_rep
                .release
                .into_iter()
                .next()
                .ok_or_else(|| "release missing in Book".to_string())?,
            first_release: intermediate_rep
                .first_release
                .into_iter()
                .next()
                .ok_or_else(|| "first_release missing in Book".to_string())?,
            authors: intermediate_rep.authors.into_iter().next(),
            generes: intermediate_rep.generes.into_iter().next(),
            series: intermediate_rep.series.into_iter().next(),
            edition: intermediate_rep
                .edition
                .into_iter()
                .next()
                .ok_or_else(|| "edition missing in Book".to_string())?,
            price: intermediate_rep
                .price
                .into_iter()
                .next()
                .ok_or_else(|| "price missing in Book".to_string())?,
            discounts: intermediate_rep.discounts.into_iter().next(),
            available: intermediate_rep
                .available
                .into_iter()
                .next()
                .ok_or_else(|| "available missing in Book".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in Book".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Book> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Book>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Book>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Book - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Book> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Book as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Book - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookProperties {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// the date when this edition of the book was released
    #[serde(rename = "release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<chrono::naive::NaiveDate>,

    /// the date when the first edition of the book was release
    #[serde(rename = "first_release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_release: Option<chrono::naive::NaiveDate>,

    #[serde(rename = "authors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,

    #[serde(rename = "generes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generes: Option<Vec<String>>,

    #[serde(rename = "discount_codes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,

    #[serde(rename = "series")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// the edition of this book
    #[serde(rename = "edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,

    /// the price of this book in Dollar
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

impl BookProperties {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> BookProperties {
        BookProperties {
            title: None,
            release: None,
            first_release: None,
            authors: None,
            generes: None,
            discount_codes: None,
            series: None,
            edition: None,
            price: None,
        }
    }
}

/// Converts the BookProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for BookProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.title
                .as_ref()
                .map(|title| ["title".to_string(), title.to_string()].join(",")),
            // Skipping release in query parameter serialization

            // Skipping first_release in query parameter serialization
            self.authors.as_ref().map(|authors| {
                [
                    "authors".to_string(),
                    authors
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.generes.as_ref().map(|generes| {
                [
                    "generes".to_string(),
                    generes
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.discount_codes.as_ref().map(|discount_codes| {
                [
                    "discount_codes".to_string(),
                    discount_codes
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.series
                .as_ref()
                .map(|series| ["series".to_string(), series.to_string()].join(",")),
            self.edition
                .as_ref()
                .map(|edition| ["edition".to_string(), edition.to_string()].join(",")),
            self.price
                .as_ref()
                .map(|price| ["price".to_string(), price.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BookProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BookProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub release: Vec<chrono::naive::NaiveDate>,
            pub first_release: Vec<chrono::naive::NaiveDate>,
            pub authors: Vec<Vec<String>>,
            pub generes: Vec<Vec<String>>,
            pub discount_codes: Vec<Vec<String>>,
            pub series: Vec<String>,
            pub edition: Vec<i32>,
            pub price: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing BookProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "release" => intermediate_rep.release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_release" => intermediate_rep.first_release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "authors" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in BookProperties"
                                .to_string(),
                        )
                    }
                    "generes" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in BookProperties"
                                .to_string(),
                        )
                    }
                    "discount_codes" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in BookProperties"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "series" => intermediate_rep.series.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "edition" => intermediate_rep.edition.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing BookProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BookProperties {
            title: intermediate_rep.title.into_iter().next(),
            release: intermediate_rep.release.into_iter().next(),
            first_release: intermediate_rep.first_release.into_iter().next(),
            authors: intermediate_rep.authors.into_iter().next(),
            generes: intermediate_rep.generes.into_iter().next(),
            discount_codes: intermediate_rep.discount_codes.into_iter().next(),
            series: intermediate_rep.series.into_iter().next(),
            edition: intermediate_rep.edition.into_iter().next(),
            price: intermediate_rep.price.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BookProperties> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BookProperties>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<BookProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for BookProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BookProperties> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <BookProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into BookProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DiscountCode {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "percentage_discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_discount: Option<i32>,

    #[serde(rename = "valid_from")]
    pub valid_from: chrono::naive::NaiveDate,

    #[serde(rename = "valid_to")]
    pub valid_to: chrono::naive::NaiveDate,

    #[serde(rename = "code")]
    pub code: String,
}

impl DiscountCode {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        valid_from: chrono::naive::NaiveDate,
        valid_to: chrono::naive::NaiveDate,
        code: String,
    ) -> DiscountCode {
        DiscountCode {
            id,
            percentage_discount: None,
            valid_from,
            valid_to,
            code,
        }
    }
}

/// Converts the DiscountCode value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DiscountCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            self.percentage_discount
                .as_ref()
                .map(|percentage_discount| {
                    [
                        "percentage_discount".to_string(),
                        percentage_discount.to_string(),
                    ]
                    .join(",")
                }),
            // Skipping valid_from in query parameter serialization

            // Skipping valid_to in query parameter serialization
            Some("code".to_string()),
            Some(self.code.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DiscountCode value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DiscountCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub percentage_discount: Vec<i32>,
            pub valid_from: Vec<chrono::naive::NaiveDate>,
            pub valid_to: Vec<chrono::naive::NaiveDate>,
            pub code: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DiscountCode".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "percentage_discount" => intermediate_rep.percentage_discount.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_from" => intermediate_rep.valid_from.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_to" => intermediate_rep.valid_to.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DiscountCode".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DiscountCode {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in DiscountCode".to_string())?,
            percentage_discount: intermediate_rep.percentage_discount.into_iter().next(),
            valid_from: intermediate_rep
                .valid_from
                .into_iter()
                .next()
                .ok_or_else(|| "valid_from missing in DiscountCode".to_string())?,
            valid_to: intermediate_rep
                .valid_to
                .into_iter()
                .next()
                .ok_or_else(|| "valid_to missing in DiscountCode".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in DiscountCode".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DiscountCode> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DiscountCode>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DiscountCode>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DiscountCode - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DiscountCode> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DiscountCode as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DiscountCode - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DiscountCodeProperties {
    #[serde(rename = "percentage_discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_discount: Option<i32>,

    #[serde(rename = "valid_from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<chrono::naive::NaiveDate>,

    #[serde(rename = "valid_to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<chrono::naive::NaiveDate>,
}

impl DiscountCodeProperties {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> DiscountCodeProperties {
        DiscountCodeProperties {
            percentage_discount: None,
            valid_from: None,
            valid_to: None,
        }
    }
}

/// Converts the DiscountCodeProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DiscountCodeProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.percentage_discount
                .as_ref()
                .map(|percentage_discount| {
                    [
                        "percentage_discount".to_string(),
                        percentage_discount.to_string(),
                    ]
                    .join(",")
                }),
            // Skipping valid_from in query parameter serialization

            // Skipping valid_to in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DiscountCodeProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DiscountCodeProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub percentage_discount: Vec<i32>,
            pub valid_from: Vec<chrono::naive::NaiveDate>,
            pub valid_to: Vec<chrono::naive::NaiveDate>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DiscountCodeProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "percentage_discount" => intermediate_rep.percentage_discount.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_from" => intermediate_rep.valid_from.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_to" => intermediate_rep.valid_to.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DiscountCodeProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DiscountCodeProperties {
            percentage_discount: intermediate_rep.percentage_discount.into_iter().next(),
            valid_from: intermediate_rep.valid_from.into_iter().next(),
            valid_to: intermediate_rep.valid_to.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DiscountCodeProperties> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DiscountCodeProperties>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DiscountCodeProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DiscountCodeProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DiscountCodeProperties> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DiscountCodeProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DiscountCodeProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Genre {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,
}

impl Genre {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, name: String) -> Genre {
        Genre { id, name }
    }
}

/// Converts the Genre value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("name".to_string()),
            Some(self.name.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Genre value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Genre {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Genre".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Genre".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Genre {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in Genre".to_string())?,
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in Genre".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Genre> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Genre>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Genre>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Genre - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Genre> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Genre as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Genre - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HealthCheckResponse {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl HealthCheckResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> HealthCheckResponse {
        HealthCheckResponse { status: None }
    }
}

/// Converts the HealthCheckResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for HealthCheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![self
            .status
            .as_ref()
            .map(|status| ["status".to_string(), status.to_string()].join(","))];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HealthCheckResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HealthCheckResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HealthCheckResponse".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HealthCheckResponse".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HealthCheckResponse {
            status: intermediate_rep.status.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HealthCheckResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<HealthCheckResponse>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HealthCheckResponse>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HealthCheckResponse - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<HealthCheckResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HealthCheckResponse as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HealthCheckResponse - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Inventory {
    #[serde(rename = "books_available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books_available: Option<i32>,

    #[serde(rename = "books_reordered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books_reordered: Option<i32>,

    #[serde(rename = "books_out_of_stock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books_out_of_stock: Option<i32>,
}

impl Inventory {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Inventory {
        Inventory {
            books_available: None,
            books_reordered: None,
            books_out_of_stock: None,
        }
    }
}

/// Converts the Inventory value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.books_available.as_ref().map(|books_available| {
                ["books_available".to_string(), books_available.to_string()].join(",")
            }),
            self.books_reordered.as_ref().map(|books_reordered| {
                ["books_reordered".to_string(), books_reordered.to_string()].join(",")
            }),
            self.books_out_of_stock.as_ref().map(|books_out_of_stock| {
                [
                    "books_out_of_stock".to_string(),
                    books_out_of_stock.to_string(),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Inventory value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Inventory {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub books_available: Vec<i32>,
            pub books_reordered: Vec<i32>,
            pub books_out_of_stock: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Inventory".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "books_available" => intermediate_rep.books_available.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "books_reordered" => intermediate_rep.books_reordered.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "books_out_of_stock" => intermediate_rep.books_out_of_stock.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Inventory".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Inventory {
            books_available: intermediate_rep.books_available.into_iter().next(),
            books_reordered: intermediate_rep.books_reordered.into_iter().next(),
            books_out_of_stock: intermediate_rep.books_out_of_stock.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Inventory> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Inventory>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Inventory>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Inventory - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Inventory> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Inventory as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Inventory - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewAuthor {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "second_names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_names: Option<Vec<String>>,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "date_of_birth")]
    pub date_of_birth: chrono::naive::NaiveDate,

    #[serde(rename = "date_of_death")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<chrono::naive::NaiveDate>,
}

impl NewAuthor {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        first_name: String,
        last_name: String,
        date_of_birth: chrono::naive::NaiveDate,
    ) -> NewAuthor {
        NewAuthor {
            title: None,
            first_name,
            second_names: None,
            last_name,
            date_of_birth,
            date_of_death: None,
        }
    }
}

/// Converts the NewAuthor value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewAuthor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.title
                .as_ref()
                .map(|title| ["title".to_string(), title.to_string()].join(",")),
            Some("first_name".to_string()),
            Some(self.first_name.to_string()),
            self.second_names.as_ref().map(|second_names| {
                [
                    "second_names".to_string(),
                    second_names
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            Some("last_name".to_string()),
            Some(self.last_name.to_string()),
            // Skipping date_of_birth in query parameter serialization

            // Skipping date_of_death in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewAuthor value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewAuthor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub first_name: Vec<String>,
            pub second_names: Vec<Vec<String>>,
            pub last_name: Vec<String>,
            pub date_of_birth: Vec<chrono::naive::NaiveDate>,
            pub date_of_death: Vec<chrono::naive::NaiveDate>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing NewAuthor".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_name" => intermediate_rep.first_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "second_names" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in NewAuthor"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "last_name" => intermediate_rep.last_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "date_of_birth" => intermediate_rep.date_of_birth.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "date_of_death" => intermediate_rep.date_of_death.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing NewAuthor".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewAuthor {
            title: intermediate_rep.title.into_iter().next(),
            first_name: intermediate_rep
                .first_name
                .into_iter()
                .next()
                .ok_or_else(|| "first_name missing in NewAuthor".to_string())?,
            second_names: intermediate_rep.second_names.into_iter().next(),
            last_name: intermediate_rep
                .last_name
                .into_iter()
                .next()
                .ok_or_else(|| "last_name missing in NewAuthor".to_string())?,
            date_of_birth: intermediate_rep
                .date_of_birth
                .into_iter()
                .next()
                .ok_or_else(|| "date_of_birth missing in NewAuthor".to_string())?,
            date_of_death: intermediate_rep.date_of_death.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewAuthor> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewAuthor>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<NewAuthor>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for NewAuthor - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewAuthor> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <NewAuthor as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into NewAuthor - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewBook {
    #[serde(rename = "title")]
    pub title: String,

    /// the date when this edition of the book was released
    #[serde(rename = "release")]
    pub release: chrono::naive::NaiveDate,

    /// the date when the first edition of the book was release
    #[serde(rename = "first_release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_release: Option<chrono::naive::NaiveDate>,

    #[serde(rename = "authors")]
    pub authors: Vec<String>,

    #[serde(rename = "generes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generes: Option<Vec<String>>,

    #[serde(rename = "discount_codes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<String>>,

    #[serde(rename = "series")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    /// the edition of this book
    #[serde(rename = "edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,

    /// the price of this book in Dollar
    #[serde(rename = "price")]
    pub price: f64,
}

impl NewBook {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        title: String,
        release: chrono::naive::NaiveDate,
        authors: Vec<String>,
        price: f64,
    ) -> NewBook {
        NewBook {
            title,
            release,
            first_release: None,
            authors,
            generes: None,
            discount_codes: None,
            series: None,
            edition: None,
            price,
        }
    }
}

/// Converts the NewBook value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("title".to_string()),
            Some(self.title.to_string()),
            // Skipping release in query parameter serialization

            // Skipping first_release in query parameter serialization
            Some("authors".to_string()),
            Some(
                self.authors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            self.generes.as_ref().map(|generes| {
                [
                    "generes".to_string(),
                    generes
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.discount_codes.as_ref().map(|discount_codes| {
                [
                    "discount_codes".to_string(),
                    discount_codes
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.series
                .as_ref()
                .map(|series| ["series".to_string(), series.to_string()].join(",")),
            self.edition
                .as_ref()
                .map(|edition| ["edition".to_string(), edition.to_string()].join(",")),
            Some("price".to_string()),
            Some(self.price.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewBook value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewBook {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub release: Vec<chrono::naive::NaiveDate>,
            pub first_release: Vec<chrono::naive::NaiveDate>,
            pub authors: Vec<Vec<String>>,
            pub generes: Vec<Vec<String>>,
            pub discount_codes: Vec<Vec<String>>,
            pub series: Vec<String>,
            pub edition: Vec<i32>,
            pub price: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing NewBook".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "release" => intermediate_rep.release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "first_release" => intermediate_rep.first_release.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "authors" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in NewBook"
                                .to_string(),
                        )
                    }
                    "generes" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in NewBook"
                                .to_string(),
                        )
                    }
                    "discount_codes" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in NewBook"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "series" => intermediate_rep.series.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "edition" => intermediate_rep.edition.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing NewBook".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewBook {
            title: intermediate_rep
                .title
                .into_iter()
                .next()
                .ok_or_else(|| "title missing in NewBook".to_string())?,
            release: intermediate_rep
                .release
                .into_iter()
                .next()
                .ok_or_else(|| "release missing in NewBook".to_string())?,
            first_release: intermediate_rep.first_release.into_iter().next(),
            authors: intermediate_rep
                .authors
                .into_iter()
                .next()
                .ok_or_else(|| "authors missing in NewBook".to_string())?,
            generes: intermediate_rep.generes.into_iter().next(),
            discount_codes: intermediate_rep.discount_codes.into_iter().next(),
            series: intermediate_rep.series.into_iter().next(),
            edition: intermediate_rep.edition.into_iter().next(),
            price: intermediate_rep
                .price
                .into_iter()
                .next()
                .ok_or_else(|| "price missing in NewBook".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewBook> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewBook>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<NewBook>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for NewBook - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewBook> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <NewBook as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into NewBook - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewDiscountCode {
    #[serde(rename = "percentage_discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_discount: Option<i32>,

    #[serde(rename = "valid_from")]
    pub valid_from: chrono::naive::NaiveDate,

    #[serde(rename = "valid_to")]
    pub valid_to: chrono::naive::NaiveDate,

    #[serde(rename = "code")]
    pub code: String,
}

impl NewDiscountCode {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        valid_from: chrono::naive::NaiveDate,
        valid_to: chrono::naive::NaiveDate,
        code: String,
    ) -> NewDiscountCode {
        NewDiscountCode {
            percentage_discount: None,
            valid_from,
            valid_to,
            code,
        }
    }
}

/// Converts the NewDiscountCode value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewDiscountCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.percentage_discount
                .as_ref()
                .map(|percentage_discount| {
                    [
                        "percentage_discount".to_string(),
                        percentage_discount.to_string(),
                    ]
                    .join(",")
                }),
            // Skipping valid_from in query parameter serialization

            // Skipping valid_to in query parameter serialization
            Some("code".to_string()),
            Some(self.code.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewDiscountCode value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewDiscountCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub percentage_discount: Vec<i32>,
            pub valid_from: Vec<chrono::naive::NaiveDate>,
            pub valid_to: Vec<chrono::naive::NaiveDate>,
            pub code: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing NewDiscountCode".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "percentage_discount" => intermediate_rep.percentage_discount.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_from" => intermediate_rep.valid_from.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "valid_to" => intermediate_rep.valid_to.push(
                        <chrono::naive::NaiveDate as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing NewDiscountCode".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewDiscountCode {
            percentage_discount: intermediate_rep.percentage_discount.into_iter().next(),
            valid_from: intermediate_rep
                .valid_from
                .into_iter()
                .next()
                .ok_or_else(|| "valid_from missing in NewDiscountCode".to_string())?,
            valid_to: intermediate_rep
                .valid_to
                .into_iter()
                .next()
                .ok_or_else(|| "valid_to missing in NewDiscountCode".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in NewDiscountCode".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewDiscountCode> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewDiscountCode>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<NewDiscountCode>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for NewDiscountCode - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewDiscountCode> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <NewDiscountCode as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into NewDiscountCode - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewOrder {
    #[serde(rename = "bookId")]
    pub book_id: String,

    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[serde(rename = "quantity")]
    pub quantity: i32,

    #[serde(rename = "shipping_date")]
    pub shipping_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "shipping_address_override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_override: Option<models::Address>,

    /// Order Status
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "complete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl NewOrder {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        book_id: String,
        customer_id: String,
        quantity: i32,
        shipping_date: chrono::DateTime<chrono::Utc>,
        status: String,
    ) -> NewOrder {
        NewOrder {
            book_id,
            customer_id,
            quantity,
            shipping_date,
            shipping_address_override: None,
            status,
            complete: None,
        }
    }
}

/// Converts the NewOrder value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("bookId".to_string()),
            Some(self.book_id.to_string()),
            Some("customerId".to_string()),
            Some(self.customer_id.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
            // Skipping shipping_date in query parameter serialization

            // Skipping shipping_address_override in query parameter serialization
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.complete
                .as_ref()
                .map(|complete| ["complete".to_string(), complete.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewOrder value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewOrder {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub book_id: Vec<String>,
            pub customer_id: Vec<String>,
            pub quantity: Vec<i32>,
            pub shipping_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub shipping_address_override: Vec<models::Address>,
            pub status: Vec<String>,
            pub complete: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing NewOrder".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "bookId" => intermediate_rep.book_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "customerId" => intermediate_rep.customer_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shipping_date" => intermediate_rep.shipping_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shipping_address_override" => intermediate_rep.shipping_address_override.push(
                        <models::Address as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "complete" => intermediate_rep.complete.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing NewOrder".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewOrder {
            book_id: intermediate_rep
                .book_id
                .into_iter()
                .next()
                .ok_or_else(|| "bookId missing in NewOrder".to_string())?,
            customer_id: intermediate_rep
                .customer_id
                .into_iter()
                .next()
                .ok_or_else(|| "customerId missing in NewOrder".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in NewOrder".to_string())?,
            shipping_date: intermediate_rep
                .shipping_date
                .into_iter()
                .next()
                .ok_or_else(|| "shipping_date missing in NewOrder".to_string())?,
            shipping_address_override: intermediate_rep
                .shipping_address_override
                .into_iter()
                .next(),
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in NewOrder".to_string())?,
            complete: intermediate_rep.complete.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewOrder> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewOrder>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<NewOrder>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for NewOrder - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewOrder> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <NewOrder as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into NewOrder - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "bookId")]
    pub book_id: String,

    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[serde(rename = "quantity")]
    pub quantity: i32,

    #[serde(rename = "shipping_date")]
    pub shipping_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "shipping_address_override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_override: Option<models::Address>,

    /// Order Status
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "complete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl Order {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        id: String,
        book_id: String,
        customer_id: String,
        quantity: i32,
        shipping_date: chrono::DateTime<chrono::Utc>,
        status: String,
    ) -> Order {
        Order {
            id,
            book_id,
            customer_id,
            quantity,
            shipping_date,
            shipping_address_override: None,
            status,
            complete: None,
        }
    }
}

/// Converts the Order value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("id".to_string()),
            Some(self.id.to_string()),
            Some("bookId".to_string()),
            Some(self.book_id.to_string()),
            Some("customerId".to_string()),
            Some(self.customer_id.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
            // Skipping shipping_date in query parameter serialization

            // Skipping shipping_address_override in query parameter serialization
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.complete
                .as_ref()
                .map(|complete| ["complete".to_string(), complete.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Order value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub book_id: Vec<String>,
            pub customer_id: Vec<String>,
            pub quantity: Vec<i32>,
            pub shipping_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub shipping_address_override: Vec<models::Address>,
            pub status: Vec<String>,
            pub complete: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Order".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "bookId" => intermediate_rep.book_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "customerId" => intermediate_rep.customer_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shipping_date" => intermediate_rep.shipping_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shipping_address_override" => intermediate_rep.shipping_address_override.push(
                        <models::Address as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "complete" => intermediate_rep.complete.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Order".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Order {
            id: intermediate_rep
                .id
                .into_iter()
                .next()
                .ok_or_else(|| "id missing in Order".to_string())?,
            book_id: intermediate_rep
                .book_id
                .into_iter()
                .next()
                .ok_or_else(|| "bookId missing in Order".to_string())?,
            customer_id: intermediate_rep
                .customer_id
                .into_iter()
                .next()
                .ok_or_else(|| "customerId missing in Order".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in Order".to_string())?,
            shipping_date: intermediate_rep
                .shipping_date
                .into_iter()
                .next()
                .ok_or_else(|| "shipping_date missing in Order".to_string())?,
            shipping_address_override: intermediate_rep
                .shipping_address_override
                .into_iter()
                .next(),
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in Order".to_string())?,
            complete: intermediate_rep.complete.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Order> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Order>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Order>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Order - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Order> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <Order as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => {
                    std::result::Result::Ok(header::IntoHeaderValue(value))
                }
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into Order - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrderProperties {
    #[serde(rename = "quantity")]
    pub quantity: i32,

    #[serde(rename = "shipping_date")]
    pub shipping_date: chrono::DateTime<chrono::Utc>,

    /// Order Status
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "complete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl OrderProperties {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        quantity: i32,
        shipping_date: chrono::DateTime<chrono::Utc>,
        status: String,
    ) -> OrderProperties {
        OrderProperties {
            quantity,
            shipping_date,
            status,
            complete: None,
        }
    }
}

/// Converts the OrderProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for OrderProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
            // Skipping shipping_date in query parameter serialization
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.complete
                .as_ref()
                .map(|complete| ["complete".to_string(), complete.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrderProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrderProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub quantity: Vec<i32>,
            pub shipping_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub status: Vec<String>,
            pub complete: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing OrderProperties".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shipping_date" => intermediate_rep.shipping_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "complete" => intermediate_rep.complete.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing OrderProperties".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrderProperties {
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in OrderProperties".to_string())?,
            shipping_date: intermediate_rep
                .shipping_date
                .into_iter()
                .next()
                .ok_or_else(|| "shipping_date missing in OrderProperties".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in OrderProperties".to_string())?,
            complete: intermediate_rep.complete.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OrderProperties> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<OrderProperties>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<OrderProperties>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for OrderProperties - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<OrderProperties> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <OrderProperties as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into OrderProperties - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
