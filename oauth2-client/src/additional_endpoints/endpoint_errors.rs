use std::error;

use crate::re_exports::{HttpError, SerdeJsonError};

#[derive(thiserror::Error, Debug)]
pub enum EndpointRenderRequestError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("Other {0}")]
    Other(Box<dyn error::Error>),
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointParseResponseError {
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
    #[error("ToUserInfoFailed {0}")]
    ToOutputFailed(String),
    //
    #[error("Other {0}")]
    Other(Box<dyn error::Error>),
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointExecuteError {
    #[error("RenderRequestError {0}")]
    RenderRequestError(EndpointRenderRequestError),
    //
    #[error("RespondFailed {0}")]
    RespondFailed(Box<dyn error::Error>),
    //
    #[error("ParseResponseError {0}")]
    ParseResponseError(EndpointParseResponseError),
}
