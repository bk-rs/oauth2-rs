//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.3.2
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.4.2
//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.4

use http::Method;
use mime::Mime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::ClientId;

pub const METHOD: Method = Method::POST;
pub const CONTENT_TYPE: Mime = mime::APPLICATION_WWW_FORM_URLENCODED;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "grant_type")]
pub enum Body {
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
    #[cfg(feature = "with-authorization-code-grant")]
    #[serde(rename = "authorization_code")]
    AuthorizationCodeGrant(BodyWithAuthorizationCodeGrant),
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.4
    #[cfg(feature = "with-device-authorization-grant")]
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceAuthorizationGrant(BodyWithDeviceAuthorizationGrant),
}

//
//
//
#[cfg(feature = "with-authorization-code-grant")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithAuthorizationCodeGrant {
    pub code: String,
    #[serde(default)]
    pub redirect_uri: Option<Url>,
    #[serde(default)]
    pub client_id: Option<ClientId>,
}

//
//
//
#[cfg(feature = "with-device-authorization-grant")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithDeviceAuthorizationGrant {
    pub device_code: String,
    #[serde(default)]
    pub client_id: Option<ClientId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "with-authorization-code-grant")]
    #[test]
    fn de_with_authorization_code_grant() {
        let body_str = "grant_type=authorization_code&code=SplxlOBeZQQYbYS6WxSbIA&redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb";
        match serde_urlencoded::from_str::<Body>(body_str) {
            Ok(Body::AuthorizationCodeGrant(body)) => {
                assert_eq!(body.code, "SplxlOBeZQQYbYS6WxSbIA");
                assert_eq!(
                    body.redirect_uri,
                    Some("https://client.example.com/cb".parse().unwrap())
                );
            }
            Ok(body) => panic!("{:?}", body),
            Err(err) => panic!("{}", err),
        }
    }

    #[cfg(feature = "with-device-authorization-grant")]
    #[test]
    fn de_with_device_authorization_grant() {
        let body_str = "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS&client_id=1406020730";
        match serde_urlencoded::from_str::<Body>(body_str) {
            Ok(Body::DeviceAuthorizationGrant(body)) => {
                assert_eq!(
                    body.device_code,
                    "GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS"
                );
                assert_eq!(body.client_id, Some("1406020730".to_owned()));
            }
            Ok(body) => panic!("{:?}", body),
            Err(err) => panic!("{}", err),
        }
    }
}
