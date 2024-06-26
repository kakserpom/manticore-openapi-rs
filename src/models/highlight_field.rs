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

/// HighlightField : Query Highlight field with options set
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HighlightField {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "limit_words", skip_serializing_if = "Option::is_none")]
    pub limit_words: Option<i32>,
    #[serde(rename = "limit_snippets", skip_serializing_if = "Option::is_none")]
    pub limit_snippets: Option<i32>,
}

impl HighlightField {
    /// Query Highlight field with options set
    pub fn new(name: String) -> HighlightField {
        HighlightField {
            name,
            limit: None,
            limit_words: None,
            limit_snippets: None,
        }
    }
}

