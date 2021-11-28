use oauth2_client::{
    authorization_code_grant::{
        FlowBuildAuthorizationUrlConfiguration, FlowBuildAuthorizationUrlError,
        FlowHandleCallbackByQueryConfiguration, FlowHandleCallbackError,
    },
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoError,
        EndpointExecuteError as UserInfoEndpointExecuteError, UserInfo,
    },
    oauth2_core::types::{CodeVerifier, Nonce, State},
};

//
//
//
pub type SigninFlowBuildAuthorizationUrlError = FlowBuildAuthorizationUrlError;

pub type SigninFlowBuildAuthorizationUrlConfiguration = FlowBuildAuthorizationUrlConfiguration;

//
#[derive(Debug, Clone, Default)]
pub struct SigninFlowHandleCallbackByQueryConfiguration {
    pub state: Option<State>,
    pub code_verifier: Option<CodeVerifier>,
    pub nonce: Option<Nonce>,
}
impl SigninFlowHandleCallbackByQueryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn set_state(&mut self, state: State) {
        self.state = Some(state);
    }

    pub fn set_code_verifier(&mut self, code_verifier: CodeVerifier) {
        self.code_verifier = Some(code_verifier);
    }

    pub fn set_nonce(&mut self, nonce: Nonce) {
        self.nonce = Some(nonce);
    }
}
impl From<SigninFlowHandleCallbackByQueryConfiguration> for FlowHandleCallbackByQueryConfiguration {
    fn from(c: SigninFlowHandleCallbackByQueryConfiguration) -> Self {
        Self {
            state: c.state.to_owned(),
            code_verifier: c.code_verifier,
        }
    }
}

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
