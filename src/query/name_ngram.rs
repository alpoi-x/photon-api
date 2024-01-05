use elasticsearch_dsl::{MultiMatchQuery, Query, TextQueryType};

pub fn build_name_ngram_query(
    q: &String,
    language: &String,
    languages: &Vec<String>,
    lenient: &bool,
) -> MultiMatchQuery {
    let default_language = if "default" == language {
        &languages[0]
    } else {
        language
    };

    let alt_names = ["alt", "int", "loc", "old", "reg", "housename"];
    let mut fields: Vec<String> = vec![];

    for lang in languages {
        let boost = if lang == default_language { 1.0 } else { 0.4 };
        fields.push(format!("name.{}.ngrams^{}", lang, boost));
    }

    for alt in alt_names {
        fields.push(format!("name.{}.raw^{}", alt, 0.4));
    }

    return Query::multi_match(fields, q.clone())
        .r#type(TextQueryType::BestFields)
        .fuzziness(if *lenient { 1 } else { 0 })
        .analyzer("search_ngram");
}
