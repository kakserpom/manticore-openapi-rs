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

/// GeoDistanceFilter : Geo distance attribute filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoDistanceFilter {
    #[serde(rename = "location_anchor", skip_serializing_if = "Option::is_none")]
    pub location_anchor: Option<Box<models::GeoDistanceFilterLocationAnchor>>,
    /// Attribute containing latitude and longitude data
    #[serde(rename = "location_source", skip_serializing_if = "Option::is_none")]
    pub location_source: Option<String>,
    #[serde(rename = "distance_type", skip_serializing_if = "Option::is_none")]
    pub distance_type: Option<DistanceType>,
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<String>,
}

impl GeoDistanceFilter {
    /// Geo distance attribute filter
    pub fn new() -> GeoDistanceFilter {
        GeoDistanceFilter {
            location_anchor: None,
            location_source: None,
            distance_type: None,
            distance: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DistanceType {
    #[serde(rename = "adaptive")]
    Adaptive,
    #[serde(rename = "haversine")]
    Haversine,
}

impl Default for DistanceType {
    fn default() -> DistanceType {
        Self::Adaptive
    }
}
