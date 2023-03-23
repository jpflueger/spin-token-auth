use anyhow::Result;
use base64::{alphabet, engine, Engine as _};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

// base64 decoding should support URL safe with no padding and allow trailing bits for JWT tokens
const BASE64_CONFIG: engine::GeneralPurposeConfig = engine::GeneralPurposeConfig::new()
    .with_decode_allow_trailing_bits(true)
    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);
const BASE64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, BASE64_CONFIG);

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonWebKey {
    #[serde(rename = "alg")]
    algorithm: String,
    #[serde(rename = "kty")]
    key_type: String,
    #[serde(rename = "use")]
    public_key_use: String,
    #[serde(rename = "n")]
    modulus: String,
    #[serde(rename = "e")]
    exponent: String,
    #[serde(rename = "kid")]
    identifier: String,
    #[serde(rename = "x5t")]
    thumbprint: String,
    #[serde(rename = "x5c")]
    chain: Vec<String>,
}

impl JsonWebKey {
    pub fn to_rsa256_public_key(self) -> Result<RS256PublicKey> {
        let n = BASE64_ENGINE.decode(self.modulus)?;
        let e = BASE64_ENGINE.decode(self.exponent)?;
        Ok(RS256PublicKey::from_components(&n, &e)?.with_key_id(self.identifier.as_str()))
    }
}
