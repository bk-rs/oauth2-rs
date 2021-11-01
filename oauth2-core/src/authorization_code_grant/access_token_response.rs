use crate::access_token_response::{GeneralErrorBody, GeneralSuccessfulBody};

pub type SuccessfulBody<SCOPE> = GeneralSuccessfulBody<SCOPE>;
pub type ErrorBody = GeneralErrorBody;
