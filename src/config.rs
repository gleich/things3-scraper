use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub endpoint: String,
    #[serde(default = "defaults::refresh")]
    pub refresh: u32,
    #[serde(default)]
    pub bearer_token: String,
}

mod defaults {
    pub fn refresh() -> u32 {
        10
    }
}

impl Config {
    pub fn parse<T: ToString>(content: T) -> Result<Self> {
        let decoded: Self =
            toml::from_str(&content.to_string()).context("failed to decode config toml")?;
        Ok(decoded)
    }

    pub fn read() -> String {
        let location = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("things3-scraper")
            .join("config.toml");
        let contents = fs::read_to_string(location)
            .context("reading config content failed")
            .unwrap();
        contents
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn parsing() {
        // basic valid config
        assert_eq!(
            Config::parse(
                r#"
        endpoint = "https://google.com/"
        "#,
            )
            .expect("failed to parse valid basic config"),
            Config {
                endpoint: String::from("https://google.com/"),
                refresh: 10,
                bearer_token: String::new()
            }
        );
        // complex valid config
        assert_eq!(
            Config::parse(
                r#"
        endpoint = "https://google.com/"
        refresh = 20
        bearer_token = "token"
        "#,
            )
            .expect("failed to parse valid basic config"),
            Config {
                endpoint: String::from("https://google.com/"),
                refresh: 20,
                bearer_token: String::from("token")
            }
        );
        // invalid config
        assert!(Config::parse("").is_err());
    }
}
