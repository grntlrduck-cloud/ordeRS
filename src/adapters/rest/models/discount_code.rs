/*
 * OrdeRS BookStore Sample Service API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscountCode {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "percentage_discount", skip_serializing_if = "Option::is_none")]
    pub percentage_discount: Option<i32>,
    #[serde(rename = "valid_from")]
    pub valid_from: String,
    #[serde(rename = "valid_to")]
    pub valid_to: String,
    #[serde(rename = "code")]
    pub code: String,
}

impl DiscountCode {
    pub fn new(id: String, valid_from: String, valid_to: String, code: String) -> DiscountCode {
        DiscountCode {
            id,
            percentage_discount: None,
            valid_from,
            valid_to,
            code,
        }
    }
}

