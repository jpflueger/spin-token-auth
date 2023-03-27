mod config;
mod jwk;
mod jwks;
mod request;

use anyhow::{anyhow, Context, Result};
use jwt_simple::prelude::*;
use serde_json::{self, Map, Value};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::{config::Config, jwks::JsonWebKeySet, request::VerificationRequest};

#[http_component]
fn handle_spin_token_auth(req: Request) -> Result<Response> {
    // get configured options - could be wizered?
    let cfg = Config::default();

    // get the claims from the request
    let res = match claims_from_request(&cfg, &req) {
        Ok(claims) => {
            let json = serde_json::to_string(&claims)?;
            http::Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Some(json.into()))?
        }
        Err(e) => http::Response::builder()
            .status(http::StatusCode::UNAUTHORIZED)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(Some(format!("Token failed authorization: {}", e).into()))?,
    };
    Ok(res)
}

fn claims_from_request(cfg: &Config, req: &Request) -> Result<JWTClaims<Map<String, Value>>> {
    // populate verification options from config
    let options = cfg.into();

    // make external call to get the json web key set for verification
    let keys = JsonWebKeySet::fetch(cfg.jwks_uri.to_owned())
        .context(format!("Failed to retrieve JWKS from {:?}", cfg.jwks_uri))?;

    // get the access token from request header
    let token = get_access_token(req.headers()).ok_or(anyhow!(
        "Failed to get access token from Authorization header"
    ))?;

    // TODO: read the verification request from body?
    let verification_request: VerificationRequest = Default::default();

    let options = VerificationOptions {
        required_subject: verification_request.required_subject,
        required_key_id: verification_request.required_key_id,
        required_public_key: verification_request.required_public_key,
        required_nonce: verification_request.required_nonce,
        ..options
    };

    keys.verify(token, Some(options))
}

fn get_access_token(headers: &http::HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")?
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
}
