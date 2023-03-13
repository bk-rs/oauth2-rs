use http_api_client_endpoint::{Body, Endpoint, Request, Response};
use oauth2_core::{
    access_token_request::{
        Body as REQ_Body, BodyWithJwtAuthorizationGrant, CONTENT_TYPE as REQ_CONTENT_TYPE,
        METHOD as REQ_METHOD,
    },
    access_token_response::{CONTENT_TYPE as RES_CONTENT_TYPE, GENERAL_ERROR_BODY_KEY_ERROR},
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    jwt_authorization_grant::access_token_response::{
        ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::Scope,
};
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

use crate::ProviderExtJwtAuthorizationGrant;

//
//
//
#[derive(Clone)]
pub struct AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    provider: &'a (dyn ProviderExtJwtAuthorizationGrant<Scope = SCOPE> + Send + Sync),
    scopes: Option<Vec<SCOPE>>,
}
impl<'a, SCOPE> AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a (dyn ProviderExtJwtAuthorizationGrant<Scope = SCOPE> + Send + Sync),
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
        //
        let mut url = self.provider.token_endpoint_url().to_owned();

        //
        self.provider.access_token_request_url_modifying(&mut url);

        //
        let mut body = BodyWithJwtAuthorizationGrant::new(
            self.provider.assertion().to_owned(),
            self.scopes.to_owned().map(Into::into),
            self.provider.client_id().map(Into::into),
        );
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
        let body = REQ_Body::<SCOPE>::JwtAuthorizationGrant(body);

        let body_str = serde_urlencoded::to_string(body)
            .map_err(AccessTokenEndpointError::SerRequestBodyFailed)?;

        //
        let mut request = Request::builder()
            .method(REQ_METHOD)
            .uri(url.as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string())
            .body(body_str.as_bytes().to_vec())
            .map_err(AccessTokenEndpointError::MakeRequestFailed)?;

        //
        self.provider.access_token_request_modifying(&mut request);

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
    #[error("MakeRequestBodyExtraFailed {0}")]
    MakeRequestBodyExtraFailed(Box<dyn std::error::Error + Send + Sync>),
    //
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
