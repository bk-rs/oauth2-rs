//! https://datatracker.ietf.org/doc/html/rfc6749#section-3.1.2

use std::{fmt, str};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use url::Url;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct RedirectUri(Url);

impl RedirectUri {
    pub fn new(url: impl AsRef<str>) -> Result<Self, String> {
        url.as_ref().parse()
    }

    pub fn url(&self) -> &Url {
        &self.0
    }
}

impl str::FromStr for RedirectUri {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(str).map_err(|err| err.to_string())?;

        Self::try_from(url)
    }
}

impl TryFrom<Url> for RedirectUri {
    type Error = String;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if !vec!["http", "https"].contains(&url.scheme()) {
            return Err(format!("Invalid scheme: {}", url.scheme()));
        }

        Ok(Self(url))
    }
}

impl<'de> Deserialize<'de> for RedirectUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RedirectUriVisitor)
    }
}
struct RedirectUriVisitor;
impl<'de> Visitor<'de> for RedirectUriVisitor {
    type Value = RedirectUri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("should be a str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Foo {
        redirect_uri: RedirectUri,
    }

    #[test]
    fn ser() {
        match serde_json::to_string(&Foo {
            redirect_uri: "https://client.example.com/cb".parse().unwrap(),
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"redirect_uri":"https://client.example.com/cb"}"#);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn de() {
        match serde_json::from_str::<Foo>(r#"{"redirect_uri":"https://client.example.com/cb"}"#) {
            Ok(v) => {
                assert_eq!(
                    v.redirect_uri,
                    "https://client.example.com/cb".parse().unwrap()
                );
            }
            Err(err) => panic!("{}", err),
        }
    }
}
