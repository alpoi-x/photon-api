mod address_type;
mod config;
mod errors;
mod osm_tag;
mod query;
mod request;
mod structs;
mod validation;

use crate::config::{load_api_config, load_language_config, LanguageConfig};
use crate::errors::PhotonError;
use crate::request::{PhotonLookupRequest, PhotonReverseRequest, PhotonSearchRequest};
use crate::validation::{
    validate_bbox, validate_lang_parameter, validate_location_bias,
    validate_reverse_request_parameters, validate_search_request_parameters,
};
use axum::extract::State;
use axum::{routing::get, Router};
use axum_extra::extract::Query;
use axum_macros::debug_handler;
use elasticsearch::http::headers::{HeaderValue, AUTHORIZATION};
use elasticsearch::http::transport::{CloudConnectionPool, TransportBuilder};
use elasticsearch::{Elasticsearch, GetParts, SearchParts};
use query::build_search_query;
use serde::{Deserialize, Deserializer};

const PHOTON_INDEX: &'static str = "photon";

#[derive(Clone)]
struct AppState {
    client: Elasticsearch,
    languages: LanguageConfig,
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
async fn health(State(app_state): State<AppState>) -> Result<(), PhotonError> {
    app_state.client.cat().health().pretty(true).send().await?;

    Ok(())
}

#[debug_handler]
async fn search(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonSearchRequest>,
) -> Result<String, PhotonError> {
    validate_search_request_parameters(&params)?;
    validate_lang_parameter(&params.lang, &app_state.languages.valid_languages)?;

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
        debug,
    } = params;

    let location_bias = validate_location_bias(lon, lat, location_bias_scale, zoom)?;
    let envelope = validate_bbox(&bbox)?;
    let language = lang.unwrap_or_else(|| app_state.languages.default_language.clone());
    let languages = app_state.languages.valid_languages.clone();

    let lenient = true; // TODO
    let size = limit.unwrap_or_else(|| 10);

    let query = build_search_query(
        q,
        language,
        languages,
        lenient,
        osm_tag,
        envelope,
        layer,
        location_bias,
        size,
    );

    println!("{:#?}", serde_json::to_string(&query));

    let response = app_state
        .client
        .search(SearchParts::Index(&[PHOTON_INDEX]))
        .body(query)
        .send()
        .await?
        .text()
        .await?;

    // TODO make the response into the photon doc format

    return Ok(response);
}

#[debug_handler]
async fn reverse(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonReverseRequest>,
) -> Result<(), PhotonError> {
    validate_reverse_request_parameters(&params)?;
    validate_lang_parameter(&params.lang, &app_state.languages.valid_languages)?;

    let language = params
        .lang
        .unwrap_or_else(|| app_state.languages.default_language.clone());

    todo!();

    Ok(())
}

#[debug_handler]
async fn lookup(
    State(app_state): State<AppState>,
    Query(params): Query<PhotonLookupRequest>,
) -> Result<String, PhotonError> {
    validate_lang_parameter(&params.lang, &app_state.languages.valid_languages)?;

    let language = params
        .lang
        .unwrap_or_else(|| app_state.languages.default_language.clone()); // TODO

    let response = app_state
        .client
        .get(GetParts::IndexId(PHOTON_INDEX, &params.place_id))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
