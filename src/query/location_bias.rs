use elasticsearch_dsl::{
    Decay, DecayFunction, Distance, Function, FunctionBoostMode, FunctionScoreMode,
    FunctionScoreQuery, GeoLocation, Query,
};

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct LocationBias {
    pub point: Point,
    pub scale: f64,
    pub zoom: i64,
}

pub fn add_location_bias(
    query: FunctionScoreQuery,
    bias: &Option<LocationBias>,
) -> FunctionScoreQuery {
    if let Some(bias) = bias {
        if bias.zoom < 4 {
            return query;
        }
        let location_bias_query = build_location_bias_query(bias);
        return location_bias_query.query(query);
    }
    return query;
}

fn build_location_bias_query(bias: &LocationBias) -> FunctionScoreQuery {
    const MIN_SCALE: f64 = 0.0000001;
    const MAX_ZOOM: i64 = 18;

    let radius = ((1 << (18 - std::cmp::min(bias.zoom, MAX_ZOOM))) / 4) as u64;

    let scale = if bias.scale < MIN_SCALE {
        MIN_SCALE
    } else {
        bias.scale
    };

    return Query::function_score()
        .function(
            Function::decay(
                DecayFunction::Exp,
                "coordinate",
                GeoLocation::new(bias.point.x, bias.point.y),
                Distance::Kilometers(radius),
            )
            .offset(Distance::Kilometers(radius / 10))
            .decay(0.8),
        )
        .function(Decay::new(DecayFunction::Linear, "importance", 1.0, scale))
        .boost_mode(FunctionBoostMode::Multiply)
        .score_mode(FunctionScoreMode::Max);
}
