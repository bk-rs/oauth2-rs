//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.2

use std::time::Duration;

use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

pub const CONTENT_TYPE: Mime = mime::APPLICATION_JSON;
pub const INTERVAL_DEFAULT: usize = 5;
pub type DeviceCode = String;
pub type UserCode = String;
pub type VerificationUri = Url;
pub type VerificationUriComplete = Url;

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulBody {
    pub device_code: DeviceCode,
    pub user_code: UserCode,
    // e.g. google
    #[serde(alias = "verification_url")]
    pub verification_uri: VerificationUri,
    #[serde(
        alias = "verification_url_complete",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_uri_complete: Option<VerificationUriComplete>,
    pub expires_in: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<usize>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}

impl SuccessfulBody {
    pub fn new(
        device_code: DeviceCode,
        user_code: UserCode,
        verification_uri: VerificationUri,
        verification_uri_complete: Option<VerificationUriComplete>,
        expires_in: usize,
        interval: Option<usize>,
    ) -> Self {
        Self {
            device_code,
            user_code,
            verification_uri,
            verification_uri_complete,
            expires_in,
            interval,
            _extensions: None,
        }
    }

    pub fn interval(&self) -> Duration {
        Duration::from_secs(self.interval.unwrap_or(INTERVAL_DEFAULT) as u64)
    }

    pub fn set_extensions(&mut self, extensions: Map<String, Value>) {
        self._extensions = Some(extensions);
    }
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        self._extensions.as_ref()
    }
}

//
//
//
pub type ErrorBody = crate::access_token_response::ErrorBody;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de() {
        let body_str = r#"
        {
            "device_code": "GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS",
            "user_code": "WDJB-MJHT",
            "verification_uri": "https://example.com/device",
            "verification_uri_complete":
                "https://example.com/device?user_code=WDJB-MJHT",
            "expires_in": 1800,
            "interval": 5
        }
        "#;
        match serde_json::from_str::<SuccessfulBody>(body_str) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }
    }
}
