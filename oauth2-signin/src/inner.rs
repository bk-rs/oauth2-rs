use std::{collections::HashMap, fmt};

use oauth2_client::{
    additional_endpoints::UserInfoEndpoint,
    authorization_code_grant::Flow,
    re_exports::{ClientId, ClientSecret, Scope, Url},
    ProviderExtAuthorizationCodeGrant,
};

use crate::HttpClient;

pub struct SigninFlowMap {
    inner: HashMap<String, SigninFlow>,
}

pub struct SigninFlow {
    flow: Flow<HttpClient>,
    provider: Box<dyn ProviderExtAuthorizationCodeGrant<Scope = String>>,
    user_info_endpoint: Box<dyn UserInfoEndpoint>,
    client_with_user_info: HttpClient,
    another_client_with_user_info: HttpClient,
}
