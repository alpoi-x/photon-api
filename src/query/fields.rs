use elasticsearch_dsl::{Fuzziness, MultiMatchQuery, Query, TextQueryType};

pub fn build_fields_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
    lenient: &bool,
) -> MultiMatchQuery {
    let mut fields: Vec<String> = vec!["collector.default^1.0".into()];

    for lang in languages {
        let boost = if lang == language { 1.0 } else { 0.6 };
        fields.push(format!("collector.{}.ngrams^{}", lang, boost));
    }

    let text_query_type = if *lenient {
        TextQueryType::BestFields
    } else {
        TextQueryType::CrossFields
    };

    let mut fields_query = Query::multi_match(fields, q.clone())
        .r#type(text_query_type)
        .prefix_length(2)
        .analyzer("search_ngram")
        .tie_breaker(0.4)
        .minimum_should_match(if *lenient { "-34%" } else { "100%" });

    if *lenient {
        fields_query = fields_query.fuzziness(Fuzziness::Auto);
    };

    return fields_query;
}
