use serde_json::{Map, Value};

use crate::re_exports::Endpoint;

use super::{EndpointParseResponseError, EndpointRenderRequestError};

//
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub uid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    //
    pub raw: Map<String, Value>,
}

//
#[derive(Debug)]
pub enum UserInfoObtainRet {
    None,
    Static(UserInfo),
    Respond(
        Box<
            dyn Endpoint<
                    RenderRequestError = EndpointRenderRequestError,
                    ParseResponseOutput = UserInfo,
                    ParseResponseError = EndpointParseResponseError,
                > + Send
                + Sync,
        >,
    ),
}
