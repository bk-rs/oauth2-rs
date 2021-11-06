use oauth2_client::{
    additional_endpoints::{AccessTokenResponseSuccessfulBody, EndpointExecuteError, UserInfo},
    authorization_code_grant::{FlowBuildAuthorizationUrlError, FlowHandleCallbackError},
};

pub type SigninFlowBuildAuthorizationUrlError = FlowBuildAuthorizationUrlError;

#[derive(Debug)]
pub enum SigninFlowHandleCallbackRet {
    Ok((AccessTokenResponseSuccessfulBody<String>, UserInfo)),
    OkButUserInfoNone(AccessTokenResponseSuccessfulBody<String>),
    OkButUserInfoObtainError(
        (
            AccessTokenResponseSuccessfulBody<String>,
            EndpointExecuteError,
        ),
    ),
    FlowHandleCallbackError(FlowHandleCallbackError),
}
