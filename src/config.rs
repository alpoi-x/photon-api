#[derive(Clone)]
pub(crate) struct ApiConfig {
    pub(crate) host_address: String,
    pub(crate) host_port: String,
    pub(crate) elastic_api_key: String,
    pub(crate) elastic_cloud_id: String,
}

#[derive(Clone)]
pub(crate) struct LanguageConfig {
    pub(crate) valid_languages: Vec<String>,
    pub(crate) default_language: String,
}

pub(crate) fn load_api_config() -> ApiConfig {
    let host_address = match std::env::var("HOST_ADDRESS") {
        Ok(address) => address,
        _ => "127.0.0.1".into(),
    };

    let host_port = match std::env::var("HOST_PORT") {
        Ok(port) => port,
        _ => "2322".into(),
    };

    let elastic_api_key = match std::env::var("ELASTIC_API_KEY") {
        Ok(api_key) => api_key,
        _ => panic!("Environment variable `ELASTIC_API_KEY` should be set"),
    };

    let elastic_cloud_id = match std::env::var("ELASTIC_CLOUD_ID") {
        Ok(id) => id,
        _ => panic!("Environment variable `ELASTIC_CLOUD_ID` should be set"),
    };

    return ApiConfig {
        host_address,
        host_port,
        elastic_api_key,
        elastic_cloud_id,
    };
}

pub(crate) fn load_language_config() -> LanguageConfig {
    let all_valid_languages = vec!["en".into(), "de".into(), "fr".into(), "it".into()];

    let valid_languages = match std::env::var("VALID_LANGUAGES") {
        Ok(languages) => languages.split(",").map(|s| s.into()).collect(),
        _ => all_valid_languages.clone(),
    };

    for language in &valid_languages {
        if !all_valid_languages.contains(&language) {
            panic!(
                "Invalid language specified in VALID_LANGUAGES: \"{}\". Allowed languages are {:?}",
                language, all_valid_languages
            );
        }
    }

    let default_language = match std::env::var("DEFAULT_LANGUAGE") {
        Ok(language) => language,
        _ => "en".into(),
    };

    if !valid_languages.contains(&default_language) {
        panic!(
            "Invalid language specified as DEFAULT_LANGUAGE: \"{}\". Allowed languages are {:?}",
            default_language, valid_languages
        );
    }

    return LanguageConfig {
        valid_languages,
        default_language,
    };
}
