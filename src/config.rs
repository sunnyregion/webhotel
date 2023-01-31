use serde::Deserialize;
use std::fs;
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
}
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}
#[derive(Deserialize,Clone,Debug)]
pub struct WebHotelInfo{
    pub web_addr:String,
    pub web_version:String,

}