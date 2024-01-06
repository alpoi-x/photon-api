mod address_type;
mod config;
mod elastic;
mod errors;
mod query;
mod request;
mod validation;
mod response;
mod doc;

use crate::config::{load_api_config, load_language_config};
use crate::errors::PhotonError;
use crate::request::{PhotonLookupRequest, PhotonReverseRequest, PhotonSearchRequest};
use crate::validation::{
    validate_bbox, validate_lang_parameter, validate_location_bias,
    validate_reverse_request_parameters, validate_search_request_parameters,
};
use axum::extract::State;
use axum::{Router, routing::get};
use axum_extra::extract::Query;
use axum_macros::debug_handler;
use elasticsearch::http::headers::{AUTHORIZATION, HeaderValue};
use elasticsearch::http::transport::{CloudConnectionPool, TransportBuilder};
use elasticsearch::Elasticsearch;
use query::build_search_query;

use crate::elastic::{send_lookup, send_photon_query};
use crate::query::build_reverse_query;
use crate::response::PhotonResponse;

const DEFAULT: &'static str = "default";

#[derive(Clone)]
struct AppState {
    client: Elasticsearch,
    languages: Vec<String>,
}

#[tokio::main]
async fn main() {
    let config = load_api_config();
    let languages = load_language_config();

    let client =
        create_elasticsearch_client(&config.elastic_cloud_id, &config.elastic_api_key).unwrap();
    let app_state = AppState { client, languages };

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

    axum::serve(listener, router).await.unwrap();
}

fn create_elasticsearch_client(
    cloud_id: &str,
    api_key: &str,
) -> Result<Elasticsearch, PhotonError> {
    let mut api_key_header: HeaderValue =
        HeaderValue::from_str(&format!("ApiKey {}", api_key)).unwrap();
    api_key_header.set_sensitive(true);

    // NB: we may want to implement our own connection pooling, as both
    // SingleNodeConnectionPool and CloudConnectionPool use a single connection
    let conn_pool = CloudConnectionPool::new(cloud_id)?;
    let transport = TransportBuilder::new(conn_pool)
        .header(AUTHORIZATION, api_key_header)
        .build()
        .unwrap();

    return Ok(Elasticsearch::new(transport));
}

#[debug_handler]
async fn health(State(app_state): State<AppState>) -> Result<String, PhotonError> {
    let response = app_state
        .client
        .cat()
        .health()
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
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

    let location_bias = validate_location_bias(lon, lat, location_bias_scale, zoom)?;
    let envelope = validate_bbox(&bbox)?;
    let language = lang.unwrap_or_else(|| DEFAULT.to_string());
    let languages = app_state.languages.clone();

    let lenient = true; // TODO
    let size = limit.unwrap_or_else(|| 10);

    let query = build_search_query(
        q,
        language.clone(),
        languages,
        lenient,
        osm_tag,
        envelope,
        layer,
        location_bias
    );

    return send_photon_query(&app_state.client, query, size, &language).await;
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
        debug: _ // TODO
    } = params;

    let language = lang.unwrap_or_else(|| DEFAULT.to_string());
    let size = limit.unwrap_or_else(|| 10);

    let query = build_reverse_query(
        lat,
        lon,
        radius,
        query_string_filter,
        distance_sort.unwrap_or_default(),
        layer,
        osm_tag
    );

    return send_photon_query(&app_state.client, query, size, &language).await;
}

#[debug_handler]
async fn lookup(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonLookupRequest>,
) -> Result<axum::Json<PhotonResponse>, PhotonError> {
    validate_lang_parameter(&params.lang, &app_state.languages)?;

    let PhotonLookupRequest { place_id, lang } = params;

    let language = lang.unwrap_or_else(|| DEFAULT.to_string());

    return send_lookup(&app_state.client, &place_id, &language).await;
}
