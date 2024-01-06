use elasticsearch_dsl::{
    BoolQuery, Decay, DecayFunction, Function, FunctionScoreMode, FunctionScoreQuery, Fuzziness,
    MatchQuery, MultiMatchQuery, Query, Search, TextQueryType, Weight,
};
use std::collections::HashSet;

use crate::query::bbox::{add_bounding_box_filter, Envelope};
use crate::query::fields::build_fields_query;
use crate::query::layer::add_layer_filter;
use crate::query::location_bias::{add_location_bias, LocationBias};
use crate::query::name_ngram::build_name_ngram_query;
use crate::query::osm_tag::add_osm_tag_filter;

pub fn build_search_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
    lenient: &bool,
    filters: &Option<HashSet<String>>,
    bbox: &Option<Envelope>,
    layers: &Option<HashSet<String>>,
    location_bias: &Option<LocationBias>,
) -> Search {
    let mut unfiltered = build_unfiltered_query(&q, &language, &languages, &lenient);
    unfiltered = add_location_bias(unfiltered, location_bias);

    let mut top_level_filter = build_top_level_filter_query(&q, &language);
    top_level_filter = add_bounding_box_filter(bbox, top_level_filter);
    top_level_filter = add_layer_filter(layers, top_level_filter);

    let mut final_query = Query::bool().must(unfiltered);
    final_query = add_osm_tag_filter(filters, final_query);
    final_query = final_query.filter(top_level_filter);

    return Search::new().query(final_query);
}

fn build_unfiltered_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
    lenient: &bool,
) -> FunctionScoreQuery {
    let fields_query = build_fields_query(q, language, languages, lenient);
    let function_score_query = build_function_score_query(q, language, languages);
    let full_name_query = build_full_name_query(q, language, lenient);

    let mut query = Query::bool();
    query = query.must(fields_query);
    query = query.should(function_score_query);
    query = query.should(full_name_query);

    let name_ngram_query = build_name_ngram_query(q, language, languages, lenient);

    query = if !(q.contains(",") || q.contains(" ")) {
        query.must(name_ngram_query.boost(2))
    } else {
        query.must(
            Query::bool()
                .should(name_ngram_query)
                .should(Query::r#match("housenumber", q.clone()).analyzer("standard"))
                .should(Query::r#match("classification", q.clone()).boost(0.1))
                .minimum_should_match(1),
        )
    };

    return Query::function_score()
        .query(query)
        .function(Decay::new(DecayFunction::Linear, "importance", 1.0, 0.6))
        .function(Weight::new(0.1).filter(Query::r#match("classification", q.clone())))
        .score_mode(FunctionScoreMode::Sum);
}

fn build_top_level_filter_query(q: &str, language: &str) -> BoolQuery {
    return Query::bool()
        .should(Query::bool().must_not(Query::exists("housenumber")))
        .should(Query::r#match("housenumber", q).analyzer("standard"))
        .should(Query::exists(format!("name.{}.raw", language)));
}

fn build_house_number_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
) -> MultiMatchQuery {
    let mut fields: Vec<String> = vec!["collector.default.raw^1.0".into()];

    for lang in languages {
        let boost = if lang == language { 1.0 } else { 0.6 };
        fields.push(format!("collector.{}.raw^{}", lang, boost));
    }

    return Query::multi_match(fields, q.clone()).r#type(TextQueryType::BestFields);
}

fn build_function_score_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
) -> FunctionScoreQuery {
    let filter_query = Query::r#match("housenumber", q.clone()).analyzer("standard");

    let weight_function = Function::weight(10f32).filter(filter_query);

    return Query::function_score()
        .query(build_house_number_query(q, language, languages))
        .boost(0.3)
        .function(weight_function);
}

fn build_full_name_query(q: &str, language: &str, lenient: &bool) -> MatchQuery {
    return Query::r#match(format!("name.{}.raw", language), q).fuzziness(if *lenient {
        Fuzziness::Auto
    } else {
        Fuzziness::Distance(0)
    });
}
