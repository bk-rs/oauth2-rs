//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.2

use std::time::Duration;

use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::access_token_response::GeneralErrorBody;

pub const CONTENT_TYPE: Mime = mime::APPLICATION_JSON;
pub const INTERVAL_DEFAULT: usize = 5;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulBody {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub expires_in: usize,
    pub interval: Option<usize>,
}
impl SuccessfulBody {
    pub fn interval(&self) -> Duration {
        Duration::from_secs(self.interval.unwrap_or_else(|| INTERVAL_DEFAULT) as u64)
    }
}

pub type ErrorBody = GeneralErrorBody;

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
