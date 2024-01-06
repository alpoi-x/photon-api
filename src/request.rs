use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct PhotonSearchRequest {
    pub q: String,
    pub lang: Option<String>,
    pub lon: Option<f32>,
    pub lat: Option<f32>,
    pub limit: Option<i64>,
    pub location_bias_scale: Option<f64>,
    pub bbox: Option<[f32; 4]>,
    pub zoom: Option<i64>,
    pub osm_tag: Option<HashSet<String>>,
    pub layer: Option<HashSet<String>>,
    pub debug: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PhotonReverseRequest {
    pub lang: Option<String>,
    pub lon: f32,
    pub lat: f32,
    pub radius: u64,
    pub query_string_filter: Option<String>,
    pub distance_sort: Option<bool>,
    pub limit: Option<i64>,
    pub osm_tag: Option<HashSet<String>>,
    pub layer: Option<HashSet<String>>,
    pub debug: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PhotonLookupRequest {
    pub place_id: String,
    pub lang: Option<String>,
}
