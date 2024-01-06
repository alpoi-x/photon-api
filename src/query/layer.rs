use elasticsearch_dsl::{BoolQuery, Query, TermsQuery};
use std::collections::HashSet;

pub fn add_layer_filter(layers: &Option<HashSet<String>>, query: BoolQuery) -> BoolQuery {
    if let Some(layers) = layers {
        let layer_query = build_layer_filter_query(layers);
        return query.filter(layer_query);
    }
    return query;
}

pub fn build_layer_filter_query(filters: &HashSet<String>) -> TermsQuery {
    return Query::terms("type", filters);
}
