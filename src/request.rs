use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct PhotonSearchRequest {
    pub(crate) q: String,
    pub(crate) lang: Option<String>,
    pub(crate) lon: Option<f32>,
    pub(crate) lat: Option<f32>,
    pub(crate) limit: Option<i64>,
    pub(crate) location_bias_scale: Option<f64>,
    pub(crate) bbox: Option<[f32; 4]>,
    pub(crate) zoom: Option<i32>,
    pub(crate) osm_tag: Option<HashSet<String>>,
    pub(crate) layer: Option<HashSet<String>>,
    pub(crate) debug: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PhotonReverseRequest {
    pub(crate) lang: Option<String>,
    pub(crate) lon: Option<f32>,
    pub(crate) lat: Option<f32>,
    pub(crate) radius: Option<f32>,
    pub(crate) query_string_filter: Option<String>,
    pub(crate) distance_sort: Option<bool>,
    pub(crate) limit: Option<i64>,
    pub(crate) osm_tag: Option<HashSet<String>>,
    pub(crate) layer: Option<HashSet<String>>,
    pub(crate) debug: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PhotonLookupRequest {
    pub(crate) place_id: String,
    pub(crate) lang: Option<String>,
}
