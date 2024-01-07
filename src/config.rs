const DEFAULT_HOST_ADDRESS: &'static str = "0.0.0.0";
const DEFAULT_HOST_PORT: &'static str = "2322";
const DEFAULT_POOL_SIZE: i8 = 50;

#[derive(Clone)]
pub struct ApiConfig {
    pub pool_size: i8,
    pub host_address: String,
    pub host_port: String,
    pub elastic_api_key: String,
    pub elastic_cloud_id: String,
}

#[derive(Clone)]
pub struct LanguageConfig {
    pub valid_languages: Vec<String>,
}

pub fn load_api_config() -> ApiConfig {
    let host_address = std::env::var("HOST_ADDRESS").unwrap_or_else(|_| DEFAULT_HOST_ADDRESS.to_string());
    let host_port = std::env::var("HOST_PORT").unwrap_or_else(|_| DEFAULT_HOST_PORT.to_string());

    let pool_size = match std::env::var("POOL_SIZE") {
        Ok(pool_size) => pool_size.parse::<i8>().unwrap_or_else(|_| DEFAULT_POOL_SIZE),
        _ => DEFAULT_POOL_SIZE
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
        pool_size,
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
