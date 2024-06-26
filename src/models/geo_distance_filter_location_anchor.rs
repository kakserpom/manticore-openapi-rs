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

/// GeoDistanceFilterLocationAnchor : Geo pin point object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoDistanceFilterLocationAnchor {
    /// Geo latitude of pin point in degrees
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    /// Geo longitude pf pin point in degrees
    #[serde(rename = "lon", skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

impl GeoDistanceFilterLocationAnchor {
    /// Geo pin point object
    pub fn new() -> GeoDistanceFilterLocationAnchor {
        GeoDistanceFilterLocationAnchor {
            lat: None,
            lon: None,
        }
    }
}

