use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PhotonResponse {
    pub r#type: String,
    pub features: Vec<PhotonResponseFeature>,
}

#[derive(Debug, Serialize)]
pub struct PhotonResponseFeature {
    pub r#type: String,
    pub properties: PhotonResponseProperties,
}

#[derive(Debug, Serialize)]
pub struct PhotonResponseProperties {
    pub parent_place_id: Option<i64>,
    pub place_id: i64,
    pub osm_type: String,
    pub osm_id: i64,
    pub osm_key: String,
    pub osm_value: String,
    pub r#type: String,
    pub postcode: Option<String>,
    pub housenumber: Option<String>,
    pub countrycode: Option<String>,
    pub name: String,
    pub country: String,
    pub city: Option<String>,
    pub district: Option<String>,
    pub locality: Option<String>,
    pub street: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub extent: Option<[f32; 4]>,
    pub extra: Option<HashMap<String, String>>,
    pub names: Option<HashMap<String, String>>,
    pub geometry: PhotonGeometry,
}

#[derive(Debug, Serialize)]
pub struct PhotonGeometry {
    pub r#type: String,
    pub coordinates: [f32; 2],
}
