/*
 * Manticore Search Client
 *
 * Сlient for Manticore Search. 
 *
 * The version of the OpenAPI document: 3.3.1
 * Contact: info@manticoresearch.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EqualsFilter : Equals attribute filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EqualsFilter {
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl EqualsFilter {
    /// Equals attribute filter
    pub fn new(field: String, value: serde_json::Value) -> EqualsFilter {
        EqualsFilter {
            field,
            value,
        }
    }
}

