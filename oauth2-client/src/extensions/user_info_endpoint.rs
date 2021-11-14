use crate::re_exports::Endpoint;

use super::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo};

//
pub type UserInfoEndpointBox = Box<
    dyn Endpoint<
            RenderRequestError = EndpointRenderRequestError,
            ParseResponseOutput = UserInfo,
            ParseResponseError = EndpointParseResponseError,
        > + Send
        + Sync,
>;
