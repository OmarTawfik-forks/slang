use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

/// A wrapper type to make sure the DSL token is written as an identifier instead of a string literal.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier {
    value: String,
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value.fmt(f);
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        return Self { value };
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        return Self {
            value: value.to_owned(),
        };
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        todo!("deserialize an identifier if current deserializer is active, or a string of not.");
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        return self.value.serialize(serializer);
    }
}
