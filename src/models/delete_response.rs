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

/// DeleteResponse : Success response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteResponse {
    #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
    pub _index: Option<String>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i32>,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<i64>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl DeleteResponse {
    /// Success response
    pub fn new() -> DeleteResponse {
        DeleteResponse {
            _index: None,
            deleted: None,
            _id: None,
            result: None,
        }
    }
}

