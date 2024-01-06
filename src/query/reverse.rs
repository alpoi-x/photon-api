use crate::query::layer::build_layer_filter_query;
use crate::query::osm_tag::add_osm_tag_filter;
use elasticsearch_dsl::{Distance, GeoDistanceSort, GeoLocation, Query, Search, SortOrder};
use std::collections::HashSet;

pub fn build_reverse_query(
    lat: &f32,
    lon: &f32,
    radius: &u64,
    query_string_filter: &Option<String>,
    distance_sort: &bool,
    layers: &Option<HashSet<String>>,
    filters: &Option<HashSet<String>>,
) -> Search {
    let geo_distance_query = Query::geo_distance(
        "coordinate",
        GeoLocation::new(*lat, *lon),
        Distance::Kilometers(*radius),
    );

    let mut query = Query::bool();
    let mut match_all = true;

    (query, match_all) = if let Some(query_string_filter) = query_string_filter {
        if !query_string_filter.trim().is_empty() {
            (
                query.must(Query::query_string(query_string_filter.as_str())),
                false,
            )
        } else {
            (query, match_all)
        }
    } else {
        (query, match_all)
    };

    (query, match_all) = if let Some(layers) = layers {
        (query.must(build_layer_filter_query(layers)), false)
    } else {
        (query, match_all)
    };

    query = add_osm_tag_filter(filters, query);

    query = if match_all {
        query.must(Query::match_all())
    } else {
        query
    };

    query = query.filter(geo_distance_query);

    let mut search = Search::new().query(query);
    search = if *distance_sort {
        search.sort(
            GeoDistanceSort::new("coordinate", GeoLocation::new(*lat, *lon)).order(SortOrder::Asc),
        )
    } else {
        search
    };

    return search;
}
