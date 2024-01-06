use std::collections::HashSet;

use crate::address_type::address_types;
use crate::errors::ValidationError;
use crate::query::{Envelope, LocationBias, Point};
use crate::request::{PhotonReverseRequest, PhotonSearchRequest};

pub fn validate_search_request_parameters(
    request: &PhotonSearchRequest,
) -> Result<(), ValidationError> {
    if let Some(lon) = &request.lon {
        validate_lon(lon)?
    }
    if let Some(lat) = &request.lat {
        validate_lat(lat)?
    }
    if let Some(layers) = &request.layer {
        validate_layers(layers)?
    }

    return Ok(());
}

pub fn validate_reverse_request_parameters(
    request: &PhotonReverseRequest,
) -> Result<(), ValidationError> {
    validate_lon(&request.lon)?;
    validate_lat(&request.lat)?;

    if let Some(layers) = &request.layer {
        validate_layers(layers)?
    }

    return Ok(());
}

pub fn validate_lang_parameter(
    language: &Option<String>,
    valid: &Vec<String>,
) -> Result<(), ValidationError> {
    if let Some(lang) = language {
        if !valid.contains(lang) {
            return Err(ValidationError::Lang {
                value: lang.clone(),
                valid: valid.clone(),
            });
        }
    }

    return Ok(());
}

fn validate_lon(lon: &f32) -> Result<(), ValidationError> {
    if lon > &180.0 || lon < &-180.0 {
        return Err(ValidationError::Lon(lon.clone()));
    }
    return Ok(());
}

fn validate_lat(lat: &f32) -> Result<(), ValidationError> {
    if lat > &90.0 || lat < &-90.0 {
        return Err(ValidationError::Lat(lat.clone()));
    }
    return Ok(());
}

pub fn validate_bbox(bbox: &Option<[f32; 4]>) -> Result<Option<Envelope>, ValidationError> {
    if let Some(bbox) = bbox {
        if bbox[0] > 180.0
            || bbox[0] < -180.0
            || bbox[2] > 180.0
            || bbox[2] < -180.0
            || bbox[1] < -90.0
            || bbox[1] > 90.0
            || bbox[3] < -90.0
            || bbox[3] > 90.0
            || bbox[0] > bbox[2]
            || bbox[1] > bbox[3]
        {
            return Err(ValidationError::Bbox(bbox.clone()));
        }

        return Ok(Some(Envelope {
            min_lon: bbox[0],
            min_lat: bbox[1],
            max_lon: bbox[2],
            max_lat: bbox[3],
        }));
    }
    return Ok(None);
}

fn validate_layers(layers: &HashSet<String>) -> Result<(), ValidationError> {
    let layer_names: Vec<String> = address_types().iter().map(|a| a.name.into()).collect();
    for layer in layers {
        if !layer_names.contains(&layer) {
            return Err(ValidationError::Layer {
                value: layer.clone(),
                valid: layer_names.clone(),
            });
        }
    }
    return Ok(());
}

pub fn validate_location_bias(
    lon: Option<f32>,
    lat: Option<f32>,
    scale: Option<f64>,
    zoom: Option<i32>,
) -> Result<Option<LocationBias>, ValidationError> {
    return match (lon, lat, scale, zoom) {
        (Some(lon), Some(lat), Some(scale), Some(zoom)) => Ok(Some(LocationBias {
            point: Point { x: lon, y: lat },
            scale,
            zoom,
        })),
        (None, None, None, None) => Ok(None),
        _ => Err(ValidationError::LocationBias),
    };
}
