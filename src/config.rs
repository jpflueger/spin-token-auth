use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};
use jwt_simple::prelude::{Duration, VerificationOptions};
use spin_sdk::config;

const JWKS_URI: &str = "jwks_uri";
const AUDIENCES: &str = "audiences";
const ISSUERS: &str = "issuers";
const MAX_VALIDITY_SECS: &str = "max_validity_secs";
const ACCEPT_FUTURE: &str = "accept_future";
const MAX_HEADER_LENGTH: &str = "max_header_length";
const MAX_TOKEN_LENGTH: &str = "max_token_length";
const TIME_TOLERANCE_SECS: &str = "time_tolerance_secs";

pub(crate) struct Config {
    pub jwks_uri: String,
    pub audiences: Option<HashSet<String>>,
    pub issuers: Option<HashSet<String>>,
    pub max_validity: Option<Duration>,
    pub accept_future: bool,
    pub max_header_length: Option<usize>,
    pub max_token_length: Option<usize>,
    pub time_tolerance: Option<Duration>,
}

impl Default for Config {
    fn default() -> Self {
        let jwks_uri = config::get(JWKS_URI)
            .expect(format!("Missing required config item '{}'", JWKS_URI).as_str());

        let issuers = config_get_set(ISSUERS).ok();
        let audiences = config_get_set(AUDIENCES).ok();
        let max_validity: Option<Duration> = config_get_duration(MAX_VALIDITY_SECS).ok();
        let accept_future = config_get_parse::<bool>(ACCEPT_FUTURE).unwrap_or(false);
        let max_header_length = config_get_parse::<usize>(MAX_HEADER_LENGTH).ok();
        let max_token_length = config_get_parse::<usize>(MAX_TOKEN_LENGTH).ok();
        let time_tolerance = config_get_duration(TIME_TOLERANCE_SECS).ok();

        Self {
            audiences,
            issuers,
            jwks_uri,
            max_validity,
            accept_future,
            max_header_length,
            max_token_length,
            time_tolerance,
        }
    }
}

impl Into<VerificationOptions> for &Config {
    fn into(self) -> VerificationOptions {
        VerificationOptions {
            accept_future: self.accept_future,
            allowed_audiences: self.audiences.to_owned(),
            allowed_issuers: self.issuers.to_owned(),
            max_header_length: self.max_header_length,
            max_token_length: self.max_token_length,
            max_validity: self.max_validity,
            time_tolerance: self.time_tolerance,
            ..Default::default()
        }
    }
}

fn config_get_parse<F: FromStr>(key: &str) -> Result<F> {
    match config::get(key) {
        Ok(val) => val
            .parse::<F>()
            .map_err(|_| anyhow!(format!("Failed to parse config value for key={}", key))),
        Err(se) => Err(se.into()),
    }
}

fn config_get_duration(key: &str) -> Result<Duration> {
    config_get_parse(key)
        .map(Duration::from_secs)
        .map_err(|se| se.into())
}

fn config_get_set(key: &str) -> Result<HashSet<String>> {
    config::get(key)
        .map(|val| {
            val.split(',')
                .map(|s| s.trim().to_string())
                .collect::<HashSet<String>>()
        })
        .map_err(|se| se.into())
}
