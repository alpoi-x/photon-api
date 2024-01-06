use elasticsearch::params::SearchType;
use elasticsearch::{Elasticsearch, GetParts, SearchParts};
use elasticsearch_dsl::Search;

use crate::doc::{document_to_feature, ElasticsearchHit, ElasticsearchResponse};
use crate::errors::PhotonError;
use crate::response::{PhotonResponse, PhotonResponseFeature};

const PHOTON_INDEX: &'static str = "photon";

pub async fn send_photon_query(
    client: &Elasticsearch,
    query: Search,
    size: i64,
    language: &String,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    let response: ElasticsearchResponse = client
        .search(SearchParts::Index(&[PHOTON_INDEX]))
        .search_type(SearchType::QueryThenFetch)
        .size(size)
        .body(query)
        .send()
        .await?
        .json()
        .await?;

    let features: Vec<PhotonResponseFeature> = response
        .hits
        .hits
        .iter()
        .filter(|hit| hit._source.is_some())
        .map(|hit| document_to_feature(&hit._source.as_ref().unwrap(), &language))
        .collect();

    let photon_response = PhotonResponse {
        r#type: "FeatureCollection".to_string(),
        features,
    };

    return Ok(axum::Json::from(photon_response));
}

pub async fn send_lookup(
    client: &Elasticsearch,
    place_id: &String,
    language: &String,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    let response: ElasticsearchHit = client
        .get(GetParts::IndexId(PHOTON_INDEX, place_id))
        .send()
        .await?
        .json()
        .await?;

    let photon_response = PhotonResponse {
        r#type: "FeatureCollection".to_string(),
        features: match response._source {
            Some(source) => vec![document_to_feature(&source, &language)],
            None => vec![],
        },
    };

    return Ok(axum::Json::from(photon_response));
}
