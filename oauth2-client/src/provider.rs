use crate::re_exports::{ClientId, ClientSecret, Scope, Url};

pub trait Provider {
    type Scope: Scope;

    fn client_id(&self) -> Option<&ClientId>;

    fn client_secret(&self) -> Option<&ClientSecret>;

    fn token_endpoint_url(&self) -> &Url;
}
