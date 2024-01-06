use elasticsearch_dsl::{BoolQuery, Query};
use std::collections::HashSet;

pub fn add_osm_tag_filter(filters: Option<HashSet<String>>, query: BoolQuery) -> BoolQuery {
    if let Some(filters) = filters {
        let osm_tag_query = build_osm_tag_filter_query(filters);
        if let Some(osm_tag_query) = osm_tag_query {
            return query.filter(osm_tag_query);
        }
        return query;
    }
    return query;
}

fn build_osm_tag_filter_query(filter_strings: HashSet<String>) -> Option<BoolQuery> {
    let filters: Vec<OsmTagFilter> = filter_strings
        .iter()
        .filter(|&filter| {
            !(filter.is_empty() || filter.clone().replace("!", "").replace(":", "").is_empty())
        })
        .map(|filter| OsmTagFilter::from(filter.clone()))
        .collect();

    let mut include_query = Query::bool();
    let mut exclude_query = Query::bool();

    for filter in filters {
        if let (None, None) = (&filter.key, &filter.value) {
            continue;
        }

        if filter.filter_type == OsmTagFilterType::None {
            continue;
        }

        if filter.filter_type == OsmTagFilterType::ExcludeValue {
            exclude_query = exclude_query.should(
                Query::bool()
                    .must(Query::term("osm_key", &filter.key.unwrap()))
                    .must_not(Query::term("osm_value", &filter.value.unwrap())),
            );
            continue;
        }

        if let (Some(key), Some(val)) = (&filter.key, &filter.value) {
            let query = Query::bool()
                .must(Query::term("osm_key", key))
                .must(Query::term("osm_value", val));

            match filter.filter_type {
                OsmTagFilterType::Include => {
                    include_query = include_query.should(query);
                }
                OsmTagFilterType::Exclude => {
                    exclude_query = exclude_query.should(query);
                }
                _ => (),
            }
            continue;
        }

        let query = match (&filter.key, &filter.value) {
            (Some(key), None) => Query::term("osm_key", key),
            (None, Some(val)) => Query::term("osm_value", val),
            _ => panic!("impossible"),
        };

        match filter.filter_type {
            OsmTagFilterType::Include => {
                include_query = include_query.should(query);
            }
            OsmTagFilterType::Exclude => {
                exclude_query = exclude_query.should(query);
            }
            _ => (),
        };
    }

    let mut tag_filter_query = Query::bool();

    tag_filter_query = if &include_query != &Query::bool() {
        tag_filter_query.must(include_query)
    } else {
        tag_filter_query
    };

    tag_filter_query = if &exclude_query != &Query::bool() {
        tag_filter_query.must_not(exclude_query)
    } else {
        tag_filter_query
    };

    if tag_filter_query == Query::bool() {
        return None;
    }

    return Some(tag_filter_query);
}

#[derive(PartialEq)]
pub enum OsmTagFilterType {
    Include,
    Exclude,
    ExcludeValue,
    None,
}

#[derive(PartialEq)]
pub struct OsmTagFilter {
    pub filter_type: OsmTagFilterType,
    pub key: Option<String>,
    pub value: Option<String>,
}

impl From<String> for OsmTagFilter {
    fn from(value: String) -> Self {
        let filter = value.trim();

        if !filter.contains(":") {
            if filter.contains("!") {
                return OsmTagFilter {
                    filter_type: OsmTagFilterType::Exclude,
                    key: Some(filter.replacen("!", "", 1)),
                    value: None,
                };
            }

            return OsmTagFilter {
                filter_type: OsmTagFilterType::Include,
                key: Some(filter.into()),
                value: None,
            };
        }

        let mut parts = filter.split(":");
        let mut key: String = parts.next().unwrap().trim().into();
        let mut val: String = parts.next().unwrap().trim().into();
        let exclude_key = key.starts_with("!");
        let exclude_val = val.starts_with("!");
        key = key.replace("!", "");
        val = val.replace("!", "");

        if exclude_val && !(key.is_empty() || val.is_empty() || exclude_key) {
            return OsmTagFilter {
                filter_type: OsmTagFilterType::ExcludeValue,
                key: Some(key),
                value: Some(val),
            };
        }

        return OsmTagFilter {
            filter_type: if exclude_key {
                OsmTagFilterType::Exclude
            } else {
                OsmTagFilterType::Include
            },
            key: if key.is_empty() { None } else { Some(key) },
            value: if val.is_empty() { None } else { Some(val) },
        };
    }
}
