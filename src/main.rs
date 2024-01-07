mod address_type;
mod config;
mod doc;
mod elastic;
mod errors;
mod query;
mod request;
mod response;
mod validation;
mod connection;

use crate::config::{load_api_config, load_language_config};
use crate::errors::{PhotonError, POOL_ERROR_TEXT};
use crate::request::{PhotonLookupRequest, PhotonReverseRequest, PhotonSearchRequest};
use crate::validation::{
    validate_bbox, validate_lang_parameter, validate_location_bias,
    validate_reverse_request_parameters, validate_search_request_parameters,
};
use crate::connection::{create_connection_pool, ElasticConnectionPool};
use crate::elastic::{send_lookup, send_photon_query};
use crate::query::build_reverse_query;
use crate::response::PhotonResponse;

use axum::extract::State;
use axum::{routing::get, Router};
use axum_extra::extract::Query;
use axum_macros::debug_handler;
use serde_json::Value;
use query::build_search_query;

const DEFAULT: &'static str = "default";

#[derive(Clone)]
struct AppState {
    pool: ElasticConnectionPool,
    languages: Vec<String>,
}

#[tokio::main]
async fn main() {
    let config = load_api_config();
    let languages = load_language_config();
    let pool = create_connection_pool(config.elastic_cloud_id, config.elastic_api_key, 30).unwrap();

    let client = pool.get().await.unwrap();
    let health_res= client
        .cat()
        .health()
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    println!("Cluster health: {:#?}", health_res);

    let app_state = AppState { pool, languages };

    let router = Router::new()
        .route("/health", get(health))
        .route("/search", get(search))
        .route("/lookup", get(lookup))
        .route("/reverse", get(reverse))
        .with_state(app_state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host_address, config.host_port))
            .await
            .unwrap();

    println!("Ready to receive requests!");

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn health(State(app_state): State<AppState>) -> Result<String, PhotonError> {
    return (&app_state)
        .pool
        .get()
        .await.map_err(|_| PhotonError::Internal(POOL_ERROR_TEXT.into()))?
        .cat()
        .health()
        .send()
        .await.map_err(|err| PhotonError::Elasticsearch(err))?
        .text()
        .await.map_err(|err| PhotonError::Elasticsearch(err));

}

#[debug_handler]
async fn search(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonSearchRequest>,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    validate_search_request_parameters(&params)?;
    validate_lang_parameter(&params.lang, &app_state.languages)?;

    let PhotonSearchRequest {
        q,
        lang,
        lon,
        lat,
        limit,
        location_bias_scale,
        bbox,
        zoom,
        osm_tag,
        layer,
        debug: _, // TODO
    } = params;

    let location_bias = validate_location_bias(&lon, &lat, &location_bias_scale, &zoom)?;
    let envelope = validate_bbox(&bbox)?;
    let language = lang.unwrap_or_else(|| DEFAULT.to_string());
    let languages = app_state.languages.clone();

    let mut lenient = false;
    let mut size = limit.unwrap_or_else(|| 10);
    size = if size > 1 {
        (size as f32 * 1.5).round() as i64
    } else {
        size
    };

    let query = build_search_query(
        &q,
        &language,
        &languages,
        &lenient,
        &osm_tag,
        &envelope,
        &layer,
        &location_bias,
    );

    let client = &app_state.pool.get().await.map_err(|_| PhotonError::Internal(POOL_ERROR_TEXT.to_string()))?;

    let mut result = send_photon_query(client, query, size, &language).await?;

    result = if result.features.is_empty() {
        lenient = true;
        let query = build_search_query(
            &q,
            &language,
            &languages,
            &lenient,
            &osm_tag,
            &envelope,
            &layer,
            &location_bias,
        );
        send_photon_query(client, query, size, &language).await?
    } else {
        result
    };

    return Ok(axum::Json::from(result));
}

#[debug_handler]
async fn reverse(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonReverseRequest>,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    validate_reverse_request_parameters(&params)?;
    validate_lang_parameter(&params.lang, &app_state.languages)?;

    let PhotonReverseRequest {
        lang,
        lon,
        lat,
        radius,
        query_string_filter,
        distance_sort,
        limit,
        osm_tag,
        layer,
        debug: _, // TODO
    } = params;

    let language = lang.unwrap_or_else(|| DEFAULT.to_string());
    let size = limit.unwrap_or_else(|| 10);

    let query = build_reverse_query(
        &lat,
        &lon,
        &radius,
        &query_string_filter,
        &distance_sort.unwrap_or_else(|| true),
        &layer,
        &osm_tag,
    );

    let client = &app_state.pool.get().await.map_err(|_| PhotonError::Internal(POOL_ERROR_TEXT.to_string()))?;
    let result = send_photon_query(client, query, size, &language).await?;

    return Ok(axum::Json::from(result));
}

#[debug_handler]
async fn lookup(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonLookupRequest>,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    validate_lang_parameter(&params.lang, &app_state.languages)?;

    let PhotonLookupRequest { place_id, lang } = params;
    let language = lang.unwrap_or_else(|| DEFAULT.to_string());
    let client = &app_state.pool.get().await.map_err(|_| PhotonError::Internal(POOL_ERROR_TEXT.to_string()))?;

    return send_lookup(client, &place_id, &language).await;
}
