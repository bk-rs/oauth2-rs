pub use http_api_endpoint::{
    self,
    http::{self, Error as HttpError},
    Body, Request, Response,
};
pub use serde;
pub use serde_enum_str;
pub use serde_json::{self, Error as SerdeJsonError, Map, Value};
pub use serde_urlencoded::{self, ser::Error as SerdeUrlencodedSerError};
pub use thiserror;
pub use url::{ParseError as UrlParseError, Url};

pub use oauth2_core::types::{AccessTokenType, ClientId, ClientSecret, RedirectUri, Scope};

pub trait Provider {
    type Scope: Scope;

    fn client_id(&self) -> Option<&ClientId>;

    fn client_secret(&self) -> Option<&ClientSecret>;

    fn token_endpoint_url(&self) -> &Url;
}
