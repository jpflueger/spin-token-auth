use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct VerificationRequest {
    pub required_subject: Option<String>,
    pub required_key_id: Option<String>,
    pub required_public_key: Option<String>,
    pub required_nonce: Option<String>,
}

impl TryInto<VerificationRequest> for Option<&Bytes> {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<VerificationRequest, Self::Error> {
        match self {
            Some(b) => serde_json::from_slice::<VerificationRequest>(b).map_err(|e| e.into()),
            None => Ok(Default::default()),
        }
    }
}
