use std::error;

use http_api_client_endpoint::{Body, Endpoint, Request, Response};
use oauth2_core::{
    access_token_request::{
        Body as REQ_Body, BodyWithClientCredentialsGrant, CONTENT_TYPE as REQ_CONTENT_TYPE,
        METHOD as REQ_METHOD,
    },
    access_token_response::{CONTENT_TYPE as RES_CONTENT_TYPE, GENERAL_ERROR_BODY_KEY_ERROR},
    client_credentials_grant::access_token_response::{
        ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody,
    },
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Error as HttpError,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::{ClientPassword, Scope},
};
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

use crate::ProviderExtClientCredentialsGrant;

//
//
//
#[derive(Clone)]
pub struct AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    provider: &'a (dyn ProviderExtClientCredentialsGrant<Scope = SCOPE> + Send + Sync),
    scopes: Option<Vec<SCOPE>>,
}
impl<'a, SCOPE> AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a (dyn ProviderExtClientCredentialsGrant<Scope = SCOPE> + Send + Sync),
        scopes: impl Into<Option<Vec<SCOPE>>>,
    ) -> Self {
        Self {
            provider,
            scopes: scopes.into(),
        }
    }
}

impl<'a, SCOPE> Endpoint for AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope + Serialize + DeserializeOwned,
{
    type RenderRequestError = AccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<SCOPE>, RES_ErrorBody>;
    type ParseResponseError = AccessTokenEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let client_password = ClientPassword::new(
            self.provider
                .client_id()
                .cloned()
                .ok_or(AccessTokenEndpointError::ClientIdMissing)?,
            self.provider
                .client_secret()
                .cloned()
                .ok_or(AccessTokenEndpointError::ClientSecretMissing)?,
        );

        let mut body = if self.provider.client_password_in_request_body() {
            BodyWithClientCredentialsGrant::new_with_client_password(
                self.scopes.to_owned().map(Into::into),
                client_password.to_owned(),
            )
        } else {
            BodyWithClientCredentialsGrant::new(self.scopes.to_owned().map(Into::into))
        };
        if let Some(extra_ret) = self.provider.access_token_request_body_extra(&body) {
            match extra_ret {
                Ok(extra) => {
                    body.set_extra(extra);
                }
                Err(err) => {
                    return Err(AccessTokenEndpointError::MakeRequestBodyExtraFailed(err));
                }
            }
        }

        //
        let body = REQ_Body::<SCOPE>::ClientCredentialsGrant(body);

        let body_str = serde_urlencoded::to_string(body)
            .map_err(AccessTokenEndpointError::SerRequestBodyFailed)?;

        let mut request = Request::builder()
            .method(REQ_METHOD)
            .uri(self.provider.token_endpoint_url().as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string());
        if !self.provider.client_password_in_request_body() {
            request = request.header(AUTHORIZATION, client_password.header_authorization());
        }
        let request = request
            .body(body_str.as_bytes().to_vec())
            .map_err(AccessTokenEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        //
        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(response.body())
                .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<SCOPE>>(response.body())
                    .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;

                return Ok(Ok(body));
            }
        }

        let body = serde_json::from_slice::<RES_ErrorBody>(response.body())
            .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
        Ok(Err(body))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenEndpointError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("ClientSecretMissing")]
    ClientSecretMissing,
    //
    #[error("MakeRequestBodyExtraFailed {0}")]
    MakeRequestBodyExtraFailed(Box<dyn error::Error + Send + Sync>),
    //
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
