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
pub struct Author {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "second_names", skip_serializing_if = "Option::is_none")]
    pub second_names: Option<Vec<String>>,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: String,
    #[serde(rename = "date_of_death", skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,
}

impl Author {
    pub fn new(id: String, first_name: String, last_name: String, date_of_birth: String) -> Author {
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

