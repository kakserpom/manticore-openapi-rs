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

/// SuccessResponse : Success response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessResponse {
    #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
    pub _index: Option<String>,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<i64>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "found", skip_serializing_if = "Option::is_none")]
    pub found: Option<bool>,
}

impl SuccessResponse {
    /// Success response
    pub fn new() -> SuccessResponse {
        SuccessResponse {
            _index: None,
            _id: None,
            created: None,
            result: None,
            found: None,
        }
    }
}

