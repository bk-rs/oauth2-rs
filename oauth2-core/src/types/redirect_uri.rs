//! https://datatracker.ietf.org/doc/html/rfc6749#section-3.1.2

use std::{fmt, str};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectUri {
    Url(Url),
    /// https://developers.google.com/identity/protocols/oauth2/native-app
    Oob,
    /// https://developers.google.com/identity/protocols/oauth2/native-app
    OobAuto,
    Other(String),
}

impl RedirectUri {
    pub fn new(url: impl AsRef<str>) -> Result<Self, String> {
        url.as_ref().parse()
    }
}

impl str::FromStr for RedirectUri {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("http://") || s.starts_with("https://") {
            let url = Url::parse(s).map_err(|err| err.to_string())?;

            Self::try_from(url)
        } else {
            match s {
                "urn:ietf:wg:oauth:2.0:oob" => Ok(Self::Oob),
                "urn:ietf:wg:oauth:2.0:oob:auto" => Ok(Self::OobAuto),
                s => Ok(Self::Other(s.to_owned())),
            }
        }
    }
}
impl fmt::Display for RedirectUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(url) => write!(f, "{}", url.as_str()),
            Self::Oob => write!(f, "urn:ietf:wg:oauth:2.0:oob"),
            Self::OobAuto => write!(f, "urn:ietf:wg:oauth:2.0:oob:auto"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl TryFrom<Url> for RedirectUri {
    type Error = String;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if !vec!["http", "https"].contains(&url.scheme()) {
            return Err(format!("Invalid scheme: {}", url.scheme()));
        }

        Ok(Self::Url(url))
    }
}

impl Serialize for RedirectUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
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
            redirect_uri: RedirectUri::Url("https://client.example.com/cb".parse().unwrap()),
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"redirect_uri":"https://client.example.com/cb"}"#);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::to_string(&Foo {
            redirect_uri: RedirectUri::Oob,
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"redirect_uri":"urn:ietf:wg:oauth:2.0:oob"}"#);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::to_string(&Foo {
            redirect_uri: RedirectUri::OobAuto,
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"redirect_uri":"urn:ietf:wg:oauth:2.0:oob:auto"}"#);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::to_string(&Foo {
            redirect_uri: RedirectUri::Other("com.example.app:redirect_uri_path".to_owned()),
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"redirect_uri":"com.example.app:redirect_uri_path"}"#);
            }
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn de() {
        match serde_json::from_str::<Foo>(r#"{"redirect_uri":"https://client.example.com/cb"}"#) {
            Ok(v) => {
                assert_eq!(
                    v.redirect_uri,
                    RedirectUri::Url("https://client.example.com/cb".parse().unwrap())
                );
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<Foo>(r#"{"redirect_uri":"urn:ietf:wg:oauth:2.0:oob"}"#) {
            Ok(v) => {
                assert_eq!(v.redirect_uri, RedirectUri::Oob);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<Foo>(r#"{"redirect_uri":"urn:ietf:wg:oauth:2.0:oob:auto"}"#) {
            Ok(v) => {
                assert_eq!(v.redirect_uri, RedirectUri::OobAuto);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<Foo>(r#"{"redirect_uri":"com.example.app:redirect_uri_path"}"#)
        {
            Ok(v) => {
                assert_eq!(
                    v.redirect_uri,
                    RedirectUri::Other("com.example.app:redirect_uri_path".to_owned())
                );
            }
            Err(err) => panic!("{err}"),
        }
    }
}
