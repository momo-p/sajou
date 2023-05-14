use std::{env, result::Result, str::FromStr};

const RUST_ENV: &str = "RUST_ENV";
const RUST_LOG: &str = "RUST_LOG";
const DISCORD_TOKEN: &str = "DISCORD_TOKEN";

#[derive(Clone)]
pub struct ServerConfig {
    pub is_production: bool,
    pub discord_token: String,
}

#[derive(Clone, Debug)]
pub enum EnvParseError {
    KeyNotFound(String),
    KeyIsEmpty(String),
    InvalidNumber(String, String),
}

impl ServerConfig {
    pub fn get_str(key: &str) -> Result<String, EnvParseError> {
        match env::var(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(EnvParseError::KeyNotFound(key.to_string())),
        }
    }

    pub fn get_num<T: FromStr>(key: &str) -> Result<T, EnvParseError>
    where
        T::Err: std::fmt::Debug,
    {
        match Self::get_str(key)?.parse::<T>() {
            Ok(num) => Ok(num),
            Err(err) => Err(EnvParseError::InvalidNumber(
                key.to_string(),
                format!("{:?}", err),
            )),
        }
    }

    pub fn load() -> Result<Self, EnvParseError> {
        dotenv::dotenv().ok();

        let is_production: bool =
            Self::get_str(&RUST_ENV).unwrap_or("".to_string()) == "production";
        let rust_log: &str = if is_production { "INFO" } else { "DEBUG" };
        let rust_log = format!("{},selectors::matching=off", rust_log);
        env::set_var(RUST_LOG, &rust_log);
        Ok(Self {
            is_production,
            discord_token: Self::get_str(&DISCORD_TOKEN).unwrap(),
        })
    }
}
