pub mod access_token_type;
pub mod scope;

pub use access_token_type::AccessTokenType;
pub use scope::{Scope, ScopeParameter};

//
//
//
pub type ClientId = String;
pub type ClientSecret = String;
