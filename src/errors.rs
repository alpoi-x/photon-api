use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

type ElasticsearchError = elasticsearch::Error;

#[derive(Debug)]
pub enum PhotonError {
    Validation(ValidationError),
    Elasticsearch(ElasticsearchError),
}

#[derive(Debug)]
pub enum ValidationError {
    Lon(f32),
    Lat(f32),
    Bbox([f32; 4]),
    Layer { value: String, valid: Vec<String> },
    Lang { value: String, valid: Vec<String> },
    LocationBias,
}

impl IntoResponse for PhotonError {
    fn into_response(self) -> Response {
        match self {
            PhotonError::Validation(err) => err.into_response(),
            PhotonError::Elasticsearch(err) => (
                match err.status_code() {
                    // elasticsearch::http::StatusCode is incompatible with axum::http::StatusCode
                    // as they use different versions of http under the hood (true as of 04/01/24)
                    Some(code) => StatusCode::from_u16(code.as_u16()).unwrap(),
                    None => StatusCode::IM_A_TEAPOT,
                },
                err.to_string(),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        return (StatusCode::BAD_REQUEST, self.to_string()).into_response();
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            ValidationError::Lon(value) => write!(f, "invalid lon \"{value:?}\". Must be in the range [-180, 180]"),
            ValidationError::Lat(value) => write!(f, "invalid lat \"{value:?}\". Must be in the range [-90, 90]"),
            ValidationError::Bbox(value) => write!(f, "invalid bbox \"{value:?}\". Expected \"min_lon,min_lat,max_lon,max_lat\" where \"lat\" is in range [-90, 90] and \"lon\" is in range [-180, 180]"),
            ValidationError::Layer{value, valid} => write!(f, "invalid layer \"{value:?}\". Allowed layers are {valid:?}"),
            ValidationError::Lang{value, valid} => write!(f, "invalid language \"{value:?}\". Allowed languages are {valid:?}"),
            ValidationError::LocationBias => write!(f, "must use all or none of lon, lat, scale, zoom")
        };
    }
}

impl Error for ValidationError {}

impl From<ElasticsearchError> for PhotonError {
    fn from(value: ElasticsearchError) -> Self {
        return PhotonError::Elasticsearch(value);
    }
}

impl From<ValidationError> for PhotonError {
    fn from(value: ValidationError) -> Self {
        return PhotonError::Validation(value);
    }
}
