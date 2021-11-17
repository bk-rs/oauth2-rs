use oauth2_client::{
    authorization_code_grant::{FlowBuildAuthorizationUrlError, FlowHandleCallbackError},
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoError,
        EndpointExecuteError as UserInfoEndpointExecuteError, UserInfo,
    },
};

//
//
//
pub type SigninFlowBuildAuthorizationUrlError = FlowBuildAuthorizationUrlError;

//
#[derive(Debug)]
pub enum SigninFlowHandleCallbackRet {
    Ok((AccessTokenResponseSuccessfulBody<String>, UserInfo)),
    OkButUserInfoNone(AccessTokenResponseSuccessfulBody<String>),
    OkButUserInfoObtainError(
        (
            AccessTokenResponseSuccessfulBody<String>,
            BuilderObtainUserInfoError,
        ),
    ),
    OkButUserInfoEndpointExecuteError(
        (
            AccessTokenResponseSuccessfulBody<String>,
            UserInfoEndpointExecuteError,
        ),
    ),
    FlowHandleCallbackError(FlowHandleCallbackError),
}
