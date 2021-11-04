use std::error;

use crate::re_exports::{HttpError, SerdeJsonError};

#[derive(thiserror::Error, Debug)]
pub enum EndpointExecuteError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("RespondFailed {0}")]
    RespondFailed(Box<dyn error::Error>),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
    #[error("ToUserInfoFailed {0}")]
    ToUserInfoFailed(String),
    //
    #[error("Other {0}")]
    Other(Box<dyn error::Error>),
}
