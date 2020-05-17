pub struct Config {
    pub base_url: String,
    pub api_key: String
}

impl Config {
    /* thanks for @link https://qiita.com/Kogia_sima/items/6899c5196813cf231054#t-intostring */
    fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Config {
        Config {
            base_url: base_url.into(),
            api_key: api_key.into()
        }
    }
}

pub fn get_config() -> Config {
    Config::new(dotenv!("BASE_URL"), dotenv!("API_KEY"))
}

