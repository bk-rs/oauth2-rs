//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.3.2
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.4.2
//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.4

use http::Method;
use mime::Mime;
use serde::{Deserialize, Serialize};
#[cfg(any(
    feature = "with-authorization-code-grant",
    feature = "with-device-authorization-grant"
))]
use serde_json::{Map, Value};
#[cfg(any(feature = "with-authorization-code-grant",))]
use url::Url;

#[cfg(feature = "with-authorization-code-grant")]
use crate::authorization_code_grant::authorization_response::Code;
#[cfg(any(
    feature = "with-authorization-code-grant",
    feature = "with-device-authorization-grant"
))]
use crate::types::{ClientId, ClientSecret};

pub const METHOD: Method = Method::POST;
pub const CONTENT_TYPE: Mime = mime::APPLICATION_WWW_FORM_URLENCODED;
pub const GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT: &str = "authorization_code";

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
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    // Note: Not in rfc6749, but usually need.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ClientSecret>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
#[cfg(feature = "with-authorization-code-grant")]
impl BodyWithAuthorizationCodeGrant {
    pub fn new(
        code: Code,
        redirect_uri: Option<Url>,
        client_id: Option<ClientId>,
        client_secret: Option<ClientSecret>,
    ) -> Self {
        Self {
            code,
            redirect_uri,
            client_id,
            client_secret,
            _extensions: None,
        }
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
#[cfg(feature = "with-device-authorization-grant")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithDeviceAuthorizationGrant {
    pub device_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    // Note: Not in rfc6749, but may need.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ClientSecret>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
#[cfg(feature = "with-device-authorization-grant")]
impl BodyWithDeviceAuthorizationGrant {
    pub fn new(
        device_code: String,
        client_id: Option<ClientId>,
        client_secret: Option<ClientSecret>,
    ) -> Self {
        Self {
            device_code,
            client_id,
            client_secret,
            _extensions: None,
        }
    }

    pub fn set_extensions(&mut self, extensions: Map<String, Value>) {
        self._extensions = Some(extensions);
    }
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        self._extensions.as_ref()
    }
}

#[cfg(feature = "with-authorization-code-grant")]
#[cfg(test)]
mod tests_with_authorization_code_grant {
    use super::*;

    #[test]
    fn ser_de() {
        let body_str = "grant_type=authorization_code&code=SplxlOBeZQQYbYS6WxSbIA&redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb";
        match serde_urlencoded::from_str::<Body>(body_str) {
            Ok(Body::AuthorizationCodeGrant(body)) => {
                assert_eq!(body.code, "SplxlOBeZQQYbYS6WxSbIA");
                assert_eq!(
                    body.redirect_uri,
                    Some("https://client.example.com/cb".parse().unwrap())
                );
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{:?}", body),
            Err(err) => panic!("{}", err),
        }
    }
}

#[cfg(feature = "with-device-authorization-grant")]
#[cfg(test)]
mod tests_with_device_authorization_grant {
    use super::*;

    #[test]
    fn ser_de() {
        let body_str = "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS&client_id=1406020730";
        match serde_urlencoded::from_str::<Body>(body_str) {
            Ok(Body::DeviceAuthorizationGrant(body)) => {
                assert_eq!(
                    body.device_code,
                    "GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS"
                );
                assert_eq!(body.client_id, Some("1406020730".to_owned()));

                assert_eq!(
                    body_str,
                    serde_urlencoded::to_string(Body::DeviceAuthorizationGrant(body)).unwrap()
                );
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{:?}", body),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn ser_de_extensions() {
        //
        let mut extensions = Map::new();
        extensions.insert("foo".to_owned(), Value::String("bar".to_owned()));
        let mut body = BodyWithDeviceAuthorizationGrant::new(
            "your_device_code".to_owned(),
            Some("your_client_id".to_owned()),
            Some("your_client_secret".to_owned()),
        );
        body.set_extensions(extensions.to_owned());
        let body = Body::DeviceAuthorizationGrant(body);
        let body_str = serde_urlencoded::to_string(body).unwrap();
        assert_eq!(body_str, "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=your_device_code&client_id=your_client_id&client_secret=your_client_secret&foo=bar");

        match serde_urlencoded::from_str::<Body>(body_str.as_str()) {
            Ok(Body::DeviceAuthorizationGrant(body)) => {
                assert_eq!(body.extensions(), Some(&extensions));
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{:?}", body),
            Err(err) => panic!("{}", err),
        }
    }
}
