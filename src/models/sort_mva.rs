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

/// SortMva : Query sort expression for MVA attributes
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SortMva {
    #[serde(rename = "attr")]
    pub attr: String,
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(rename = "mode")]
    pub mode: Mode,
}

impl SortMva {
    /// Query sort expression for MVA attributes
    pub fn new(attr: String, order: Order, mode: Mode) -> SortMva {
        SortMva {
            attr,
            order,
            mode,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for Order {
    fn default() -> Order {
        Self::Asc
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Min
    }
}
