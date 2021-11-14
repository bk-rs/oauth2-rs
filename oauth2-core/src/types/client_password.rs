//! https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1

use std::str;

use serde::{Deserialize, Serialize};

use crate::types::{ClientId, ClientSecret};

pub const HEADER_AUTHORIZATION_PREFIX: &str = "Basic ";

//
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClientPassword {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
}

impl ClientPassword {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    pub fn header_authorization(&self) -> String {
        format!(
            "{}{}",
            HEADER_AUTHORIZATION_PREFIX,
            base64::encode(format!("{}:{}", self.client_id, self.client_secret))
        )
    }

    pub fn from_header_authorization(s: impl AsRef<str>) -> Result<Self, &'static str> {
        let s = s.as_ref();

        if !s.starts_with(HEADER_AUTHORIZATION_PREFIX) {
            return Err("Missing prefix");
        }

        let bytes =
            base64::decode(&s[HEADER_AUTHORIZATION_PREFIX.len()..][..]).map_err(|_| "Invalid")?;
        let s = str::from_utf8(&bytes).map_err(|_| "Invalid")?;

        let mut s = s.split(':');
        let client_id = s.next().ok_or("Missing client_id")?.to_owned();
        let client_secret = s.next().ok_or("Missing client_secret")?.to_owned();
        if s.next().is_some() {
            return Err("Invalid");
        }

        Ok(Self {
            client_id,
            client_secret,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_authorization() {
        assert_eq!(
            ClientPassword::new("s6BhdRkqt3".to_owned(), "7Fjfp0ZBr1KtDRbnfVdmIw".to_owned())
                .header_authorization(),
            "Basic czZCaGRSa3F0Mzo3RmpmcDBaQnIxS3REUmJuZlZkbUl3"
        );

        assert_eq!(
            ClientPassword::from_header_authorization(
                "Basic czZCaGRSa3F0Mzo3RmpmcDBaQnIxS3REUmJuZlZkbUl3"
            )
            .unwrap(),
            ClientPassword::new("s6BhdRkqt3".to_owned(), "7Fjfp0ZBr1KtDRbnfVdmIw".to_owned())
        );
    }
}
