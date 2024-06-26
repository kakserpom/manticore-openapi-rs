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

/// InsertDocumentRequest : Object with document data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertDocumentRequest {
    /// Name of the index
    #[serde(rename = "index")]
    pub index: String,
    /// cluster name
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Document ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Object with document data
    #[serde(rename = "doc")]
    pub doc: std::collections::HashMap<String, serde_json::Value>,
}

impl InsertDocumentRequest {
    /// Object with document data.
    pub fn new(
        index: String,
        doc: std::collections::HashMap<String, serde_json::Value>,
        id: Option<u64>,
    ) -> InsertDocumentRequest {
        InsertDocumentRequest {
            index,
            cluster: None,
            id,
            doc,
        }
    }
}
