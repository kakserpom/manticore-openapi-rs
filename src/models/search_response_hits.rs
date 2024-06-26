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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResponseHits {
    #[serde(rename = "max_score", skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "total_relation", skip_serializing_if = "Option::is_none")]
    pub total_relation: Option<String>,
    #[serde(rename = "hits", skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<serde_json::Value>>,
}

impl SearchResponseHits {
    pub fn new() -> SearchResponseHits {
        SearchResponseHits {
            max_score: None,
            total: None,
            total_relation: None,
            hits: None,
        }
    }
}

