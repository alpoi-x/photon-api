use crate::response::{PhotonGeometry, PhotonResponseFeature, PhotonResponseProperties};
use serde::Deserialize;
use std::collections::HashMap;

type LanguageField = HashMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct PhotonDocument {
    pub r#type: String,
    pub importance: f64,
    pub place_id: i64,
    pub parent_place_id: Option<i64>,
    pub osm_id: i64,
    pub osm_type: String,
    pub osm_value: String,
    pub osm_key: String,
    pub coordinate: PhotonDocumentCoordinate,
    pub extent: Option<PhotonDocumentExtent>,
    pub classification: Option<String>,
    pub countrycode: Option<String>,
    pub housenumber: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<LanguageField>,
    pub county: Option<LanguageField>,
    pub city: Option<LanguageField>,
    pub state: Option<LanguageField>,
    pub district: Option<LanguageField>,
    pub locality: Option<LanguageField>,
    pub street: Option<LanguageField>,
    pub name: Option<LanguageField>,
    pub names: Option<HashMap<String, String>>,
    pub extra: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct PhotonDocumentExtent {
    pub r#type: String,
    pub coordinates: [[f32; 2]; 2],
}

#[derive(Debug, Deserialize)]
pub struct PhotonDocumentCoordinate {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Deserialize)]
pub struct ElasticsearchResponse {
    pub hits: ElasticsearchHits,
}

#[derive(Debug, Deserialize)]
pub struct ElasticsearchHits {
    pub hits: Vec<ElasticsearchHit>,
}

#[derive(Debug, Deserialize)]
pub struct ElasticsearchHit {
    pub _source: Option<PhotonDocument>,
}

pub fn document_to_feature(doc: &PhotonDocument, language: &String) -> PhotonResponseFeature {
    return PhotonResponseFeature {
        r#type: "Feature".to_string(),
        properties: PhotonResponseProperties {
            parent_place_id: doc.parent_place_id,
            place_id: doc.place_id,
            osm_type: doc.osm_type.clone(),
            osm_id: doc.osm_id,
            osm_key: doc.osm_key.clone(),
            osm_value: doc.osm_value.clone(),
            r#type: doc.r#type.clone(),
            postcode: doc.postcode.clone(),
            housenumber: doc.housenumber.clone(),
            countrycode: doc.countrycode.clone(),
            name: unwrap_language_field(&doc.name, language),
            country: unwrap_language_field(&doc.country, language),
            city: unwrap_language_field(&doc.city, language),
            district: unwrap_language_field(&doc.district, language),
            locality: unwrap_language_field(&doc.locality, language),
            street: unwrap_language_field(&doc.street, language),
            state: unwrap_language_field(&doc.state, language),
            county: unwrap_language_field(&doc.county, language),
            extra: doc.extra.clone(),
            names: doc.names.clone(),
            extent: match &doc.extent {
                Some(extent) => Some([
                    extent.coordinates[0][0],
                    extent.coordinates[0][1],
                    extent.coordinates[1][0],
                    extent.coordinates[1][1],
                ]),
                None => None,
            },
            geometry: PhotonGeometry {
                r#type: "Point".to_string(),
                coordinates: [doc.coordinate.lon, doc.coordinate.lat],
            },
        },
    };
}

fn unwrap_language_field(field: &Option<LanguageField>, language: &String) -> Option<String> {
    return match field {
        Some(field) => match field.get(language) {
            Some(value) => Some(value.clone()),
            None => match field.get("default") {
                Some(value) => Some(value.clone()),
                None => None,
            },
        },
        None => None,
    };
}
