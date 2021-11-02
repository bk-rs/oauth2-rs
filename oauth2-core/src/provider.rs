pub use http::{self, Error as HttpError, Request, Response};
pub use serde_json::{Map, Value};
pub use url::{ParseError as UrlParseError, Url};

use crate::types::{ClientId, ClientSecret, Scope};

pub trait Provider {
    type Scope: Scope;

    fn client_id(&self) -> Option<&ClientId>;

    fn client_secret(&self) -> Option<&ClientSecret>;

    fn token_endpoint_url(&self) -> &Url;
}
