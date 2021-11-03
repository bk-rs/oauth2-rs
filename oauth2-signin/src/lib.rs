use std::collections::HashMap;

use oauth2_client::{oauth2_core::types::Scope, Provider};

pub struct ProviderMap {
    inner: HashMap<String, String>,
}
