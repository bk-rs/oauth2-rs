//! https://datatracker.ietf.org/doc/html/rfc6749#section-3.3

use std::{
    cmp, error, fmt,
    marker::PhantomData,
    str::{self, FromStr as _},
};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub const SCOPE_PARAMETER_DELIMITATION: char = ' ';

pub const SCOPE_OPENID: &str = "openid";

//
pub trait Scope: str::FromStr + ToString + fmt::Debug + Clone + cmp::PartialEq {}

impl Scope for String {}

//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeParameter<T>(pub Vec<T>);

impl<T> From<Vec<T>> for ScopeParameter<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}

impl<T> str::FromStr for ScopeParameter<T>
where
    T: Scope,
{
    type Err = ScopeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_char = if s.contains(SCOPE_PARAMETER_DELIMITATION) {
            SCOPE_PARAMETER_DELIMITATION
        } else if s.contains(',') {
            // e.g. github access_token_response
            ','
        } else {
            SCOPE_PARAMETER_DELIMITATION
        };

        let mut inner = vec![];
        for s in s.split(split_char) {
            inner.push(T::from_str(s).map_err(|_| ScopeFromStrError::Unknown(s.to_owned()))?);
        }
        Ok(inner.into())
    }
}

impl<T> Serialize for ScopeParameter<T>
where
    T: Scope,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(SCOPE_PARAMETER_DELIMITATION.to_string().as_str())
                .as_str(),
        )
    }
}
impl<'de, T> Deserialize<'de> for ScopeParameter<T>
where
    T: Scope,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ScopeParameterVisitor {
            phantom: PhantomData,
        })
    }
}

struct ScopeParameterVisitor<T> {
    phantom: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for ScopeParameterVisitor<T>
where
    T: Scope,
{
    type Value = ScopeParameter<T>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("should be a str or seq")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ScopeParameter::<T>::from_str(s).map_err(de::Error::custom)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut inner = vec![];
        while let Some(s) = seq.next_element::<&str>()? {
            inner.push(
                T::from_str(s)
                    .map_err(|_| de::Error::custom(ScopeFromStrError::Unknown(s.to_owned())))?,
            );
        }
        Ok(inner.into())
    }
}

//
impl<T> From<&ScopeParameter<T>> for ScopeParameter<String>
where
    T: Scope,
{
    fn from(v: &ScopeParameter<T>) -> Self {
        v.0.iter().map(|y| y.to_string()).collect::<Vec<_>>().into()
    }
}

impl<T> ScopeParameter<T>
where
    T: Scope,
{
    pub fn try_from_t_with_string(v: &ScopeParameter<String>) -> Result<Self, ScopeFromStrError> {
        let mut inner = vec![];
        for s in v.0.iter() {
            inner.push(T::from_str(s).map_err(|_| ScopeFromStrError::Unknown(s.to_owned()))?);
        }
        Ok(inner.into())
    }
}

#[derive(Debug)]
pub enum ScopeFromStrError {
    Unknown(String),
}
impl core::fmt::Display for ScopeFromStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error::Error for ScopeFromStrError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Foo {
        scope: ScopeParameter<String>,
    }

    #[test]
    fn de_and_ser() {
        match serde_json::from_str::<Foo>(r#"{"scope":"a b"}"#) {
            Ok(v) => {
                assert_eq!(v.scope, vec!["a".to_owned(), "b".to_owned()].into());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<Foo>(r#"{"scope":"a,b"}"#) {
            Ok(v) => {
                assert_eq!(v.scope, vec!["a".to_owned(), "b".to_owned()].into());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<Foo>(r#"{"scope":["a", "b"]}"#) {
            Ok(v) => {
                assert_eq!(v.scope, vec!["a".to_owned(), "b".to_owned()].into());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::to_string(&Foo {
            scope: vec!["a".to_owned(), "b".to_owned()].into(),
        }) {
            Ok(v) => {
                assert_eq!(v, r#"{"scope":"a b"}"#);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
