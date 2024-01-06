use elasticsearch_dsl::{BoolQuery, GeoBoundingBox, GeoBoundingBoxQuery, Query};

pub struct Envelope {
    pub min_lon: f32,
    pub min_lat: f32,
    pub max_lon: f32,
    pub max_lat: f32,
}

pub fn add_bounding_box_filter(bbox: Option<Envelope>, query: BoolQuery) -> BoolQuery {
    if let Some(envelope) = bbox {
        let bbox_query = build_bbox_query(envelope);
        return query.filter(bbox_query);
    }
    return query;
}

fn build_bbox_query(bbox: Envelope) -> GeoBoundingBoxQuery {
    return Query::geo_bounding_box(
        "coordinate",
        GeoBoundingBox::Vertices {
            top: bbox.min_lat,
            left: bbox.min_lon,
            bottom: bbox.max_lat,
            right: bbox.max_lon,
        },
    );
}
