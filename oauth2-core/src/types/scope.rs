//! https://datatracker.ietf.org/doc/html/rfc6749#section-3.3

use std::{cmp, error, fmt, marker::PhantomData, str};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub const SCOPE_PARAMETER_DELIMITATION: char = ' ';

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

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("should be a str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let split_char = if v.contains(SCOPE_PARAMETER_DELIMITATION) {
            SCOPE_PARAMETER_DELIMITATION
        } else if v.contains(",") {
            // e.g. github access_token_response
            ','
        } else {
            SCOPE_PARAMETER_DELIMITATION
        };

        let mut inner = vec![];
        for s in v.split(split_char) {
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
impl fmt::Display for ScopeFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl error::Error for ScopeFromStrError {}
