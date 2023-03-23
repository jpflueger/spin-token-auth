use anyhow::{bail, Result};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use spin_sdk::outbound_http;

use crate::jwk::JsonWebKey;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonWebKeySet {
    keys: Vec<JsonWebKey>,
}

impl JsonWebKeySet {
    pub fn get(url: String) -> Result<Self> {
        let res = outbound_http::send_request(
            http::Request::builder().method("GET").uri(url).body(None)?,
        )?;
        let res_body = match res.body().as_ref() {
            Some(bytes) => bytes.slice(..),
            None => bytes::Bytes::default(),
        };
        Ok(serde_json::from_slice::<JsonWebKeySet>(&res_body)?)
    }

    pub fn verify(
        self,
        token: &str,
        options: Option<VerificationOptions>,
    ) -> Result<JWTClaims<NoCustomClaims>> {
        for key in self.keys {
            let key = key.to_rsa256_public_key()?;

            // add a required key id verification to options
            let options = options.clone().map(|o| VerificationOptions {
                // ensure the token is validated by this key specifically
                required_key_id: key.key_id().to_owned(),
                ..o
            });

            let claims = key.verify_token::<NoCustomClaims>(token, options);
            if claims.is_ok() {
                return claims;
            }
        }
        bail!("No key in the set was able to verify the token.")
    }
}
