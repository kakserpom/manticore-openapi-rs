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

/// DeleteDocumentRequest : Payload for delete request. Documents can be deleted either one by one by specifying the document id or by providing a query object. For more information see  [Delete API](https://manual.manticoresearch.com/Deleting_documents) 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteDocumentRequest {
    /// Index name
    #[serde(rename = "index")]
    pub index: String,
    /// cluster name
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Document ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Query tree object
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<serde_json::Value>,
}

impl DeleteDocumentRequest {
    /// Payload for delete request. Documents can be deleted either one by one by specifying the document id or by providing a query object. For more information see  [Delete API](https://manual.manticoresearch.com/Deleting_documents) 
    pub fn new(index: String) -> DeleteDocumentRequest {
        DeleteDocumentRequest {
            index,
            cluster: None,
            id: None,
            query: None,
        }
    }
}
