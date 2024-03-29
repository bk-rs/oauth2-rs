//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.3.2
//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.4.2
//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.4
//! https://datatracker.ietf.org/doc/html/rfc7523#section-2.1

use http::Method;
use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{
    ClientId, ClientPassword, ClientSecret, Code, CodeVerifier, Scope, ScopeFromStrError,
    ScopeParameter,
};

pub const METHOD: Method = Method::POST;
pub const CONTENT_TYPE: Mime = mime::APPLICATION_WWW_FORM_URLENCODED;
pub const GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT: &str = "authorization_code";

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "grant_type")]
pub enum Body<SCOPE>
where
    SCOPE: Scope,
{
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
    #[serde(rename = "authorization_code")]
    AuthorizationCodeGrant(BodyWithAuthorizationCodeGrant),
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.4
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceAuthorizationGrant(BodyWithDeviceAuthorizationGrant),
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.4
    #[serde(rename = "client_credentials")]
    ClientCredentialsGrant(BodyWithClientCredentialsGrant<SCOPE>),
    #[serde(rename = "password")]
    ResourceOwnerPasswordCredentialsGrant(BodyWithResourceOwnerPasswordCredentialsGrant<SCOPE>),
    /// https://datatracker.ietf.org/doc/html/rfc7523#section-2.1
    #[serde(rename = "urn:ietf:params:oauth:grant-type:jwt-bearer")]
    JwtAuthorizationGrant(BodyWithJwtAuthorizationGrant<SCOPE>),
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithAuthorizationCodeGrant {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    // Note: Not in rfc6749, but usually need.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ClientSecret>,

    // PKCE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<CodeVerifier>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl BodyWithAuthorizationCodeGrant {
    pub fn new(
        code: Code,
        redirect_uri: Option<String>,
        client_id: Option<ClientId>,
        client_secret: Option<ClientSecret>,
    ) -> Self {
        Self::internal_new(code, redirect_uri, client_id, client_secret, None)
    }

    fn internal_new(
        code: Code,
        redirect_uri: Option<String>,
        client_id: Option<ClientId>,
        client_secret: Option<ClientSecret>,
        code_verifier: Option<CodeVerifier>,
    ) -> Self {
        Self {
            code,
            redirect_uri,
            client_id,
            client_secret,
            code_verifier,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithDeviceAuthorizationGrant {
    pub device_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    // Note: Not in rfc6749, but may need.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<ClientSecret>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

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
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithClientCredentialsGrant<SCOPE>
where
    SCOPE: Scope,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub client_password: Option<ClientPassword>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl<SCOPE> BodyWithClientCredentialsGrant<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(scope: Option<ScopeParameter<SCOPE>>) -> Self {
        Self {
            scope,
            client_password: None,
            _extra: None,
        }
    }

    pub fn new_with_client_password(
        scope: Option<ScopeParameter<SCOPE>>,
        client_password: ClientPassword,
    ) -> Self {
        Self {
            scope,
            client_password: Some(client_password),
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(
        body: &BodyWithClientCredentialsGrant<String>,
    ) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &body.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut this = Self::new(scope);
        this.client_password = body.client_password.to_owned();
        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithResourceOwnerPasswordCredentialsGrant<SCOPE>
where
    SCOPE: Scope,
{
    pub username: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub client_password: Option<ClientPassword>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl<SCOPE> BodyWithResourceOwnerPasswordCredentialsGrant<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        scope: Option<ScopeParameter<SCOPE>>,
    ) -> Self {
        Self {
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope,
            client_password: None,
            _extra: None,
        }
    }

    pub fn new_with_client_password(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        scope: Option<ScopeParameter<SCOPE>>,
        client_password: ClientPassword,
    ) -> Self {
        Self {
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope,
            client_password: Some(client_password),
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(
        body: &BodyWithResourceOwnerPasswordCredentialsGrant<String>,
    ) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &body.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut this = Self::new(&body.username, &body.password, scope);
        this.client_password = body.client_password.to_owned();
        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyWithJwtAuthorizationGrant<SCOPE>
where
    SCOPE: Scope,
{
    pub assertion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl<SCOPE> BodyWithJwtAuthorizationGrant<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        assertion: String,
        scope: Option<ScopeParameter<SCOPE>>,
        client_id: Option<ClientId>,
    ) -> Self {
        Self {
            assertion,
            scope,
            client_id,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(
        body: &BodyWithJwtAuthorizationGrant<String>,
    ) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &body.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut this = Self::new(body.assertion.to_owned(), scope, body.client_id.to_owned());

        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

#[cfg(test)]
mod tests_with_authorization_code_grant {
    use super::*;

    #[test]
    fn test_ser_de() {
        let body_str = "grant_type=authorization_code&code=SplxlOBeZQQYbYS6WxSbIA&redirect_uri=https%3A%2F%2Fclient%2Eexample%2Ecom%2Fcb";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::AuthorizationCodeGrant(body)) => {
                assert_eq!(body.code, "SplxlOBeZQQYbYS6WxSbIA");
                assert_eq!(
                    body.redirect_uri,
                    Some("https://client.example.com/cb".parse().unwrap())
                );
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod tests_with_device_authorization_grant {
    use super::*;

    #[test]
    fn test_ser_de() {
        let body_str = "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS&client_id=1406020730";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::DeviceAuthorizationGrant(body)) => {
                assert_eq!(
                    body.device_code,
                    "GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS"
                );
                assert_eq!(body.client_id, Some("1406020730".to_owned()));

                assert_eq!(
                    body_str,
                    serde_urlencoded::to_string(Body::<String>::DeviceAuthorizationGrant(body))
                        .unwrap()
                );
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_ser_de_extra() {
        //
        let mut extra = Map::new();
        extra.insert("foo".to_owned(), Value::String("bar".to_owned()));
        let mut body = BodyWithDeviceAuthorizationGrant::new(
            "your_device_code".to_owned(),
            Some("your_client_id".to_owned()),
            Some("your_client_secret".to_owned()),
        );
        body.set_extra(extra.to_owned());
        let body = Body::<String>::DeviceAuthorizationGrant(body);
        let body_str = serde_urlencoded::to_string(body).unwrap();
        assert_eq!(body_str, "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=your_device_code&client_id=your_client_id&client_secret=your_client_secret&foo=bar");

        match serde_urlencoded::from_str::<Body<String>>(body_str.as_str()) {
            Ok(Body::DeviceAuthorizationGrant(body)) => {
                assert_eq!(body.extra(), Some(&extra));
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod tests_with_client_credentials_grant {
    use super::*;

    #[test]
    fn test_ser_de() {
        let body_str = "grant_type=client_credentials";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ClientCredentialsGrant(body)) => {
                assert_eq!(body.client_password, None);
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }

        let body_str =
            "grant_type=client_credentials&client_id=CLIENT_ID&client_secret=CLIENT_SECRET";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ClientCredentialsGrant(body)) => {
                assert_eq!(body.client_password.unwrap().client_id, "CLIENT_ID");
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }

        let body_str = "grant_type=client_credentials&client_id=CLIENT_ID";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ClientCredentialsGrant(body)) => {
                assert_eq!(body.client_password, None);
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_ser_de_extra() {
        let body_str = "grant_type=client_credentials&foo=bar";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ClientCredentialsGrant(body)) => {
                assert_eq!(body.client_password, None);
                assert_eq!(
                    body.extra().unwrap().get("foo").unwrap().as_str(),
                    Some("bar")
                )
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }

        let body_str =
            "grant_type=client_credentials&client_id=CLIENT_ID&client_secret=CLIENT_SECRET&foo=bar";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ClientCredentialsGrant(body)) => {
                assert_eq!(
                    body.client_password.to_owned().unwrap().client_id,
                    "CLIENT_ID"
                );
                assert_eq!(
                    body.extra().unwrap().get("foo").unwrap().as_str(),
                    Some("bar")
                )
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod tests_with_resource_owner_password_credentials_grant {
    use super::*;

    #[test]
    fn test_ser_de() {
        let body_str = "grant_type=password&username=USERNAME&password=PASSWORD";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ResourceOwnerPasswordCredentialsGrant(body)) => {
                assert_eq!(body.username, "USERNAME");
                assert_eq!(body.password, "PASSWORD");
                assert_eq!(body.client_password, None);
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }

        let body_str =
            "grant_type=password&username=USERNAME&password=PASSWORD&client_id=CLIENT_ID&client_secret=CLIENT_SECRET";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ResourceOwnerPasswordCredentialsGrant(body)) => {
                assert_eq!(body.username, "USERNAME");
                assert_eq!(body.password, "PASSWORD");
                assert_eq!(body.client_password.unwrap().client_id, "CLIENT_ID");
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }

        let body_str =
            "grant_type=password&username=USERNAME&password=PASSWORD&client_id=CLIENT_ID";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::ResourceOwnerPasswordCredentialsGrant(body)) => {
                assert_eq!(body.username, "USERNAME");
                assert_eq!(body.password, "PASSWORD");
                assert_eq!(body.client_password, None);
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod tests_with_jwt_authorization_grant {
    use super::*;

    #[test]
    fn test_ser_de() {
        let body_str = "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&assertion=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(Body::JwtAuthorizationGrant(body)) => {
                assert_eq!(
                    body.assertion,
                    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
                );
                assert_eq!(body.scope, None);
                assert_eq!(body.client_id, None);

                assert_eq!(
                    body_str,
                    serde_urlencoded::to_string(Body::<String>::JwtAuthorizationGrant(body))
                        .unwrap()
                );
            }
            #[allow(unreachable_patterns)]
            Ok(body) => panic!("{body:?}"),
            Err(err) => panic!("{err}"),
        }
    }
}
