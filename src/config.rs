#[derive(Clone)]
pub struct ApiConfig {
    pub host_address: String,
    pub host_port: String,
    pub elastic_api_key: String,
    pub elastic_cloud_id: String,
}

#[derive(Clone)]
pub struct LanguageConfig {
    pub valid_languages: Vec<String>
}

pub fn load_api_config() -> ApiConfig {
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

pub fn load_language_config() -> Vec<String> {
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

    return valid_languages;
}
