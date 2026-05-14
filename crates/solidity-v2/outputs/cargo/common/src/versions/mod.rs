#[path = "language_versions.generated.rs"]
mod language_versions;

pub use language_versions::{FromSemverError, LanguageVersion};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LanguageVersionSpecifier {
    From {
        from: LanguageVersion,
    },
    Till {
        till: LanguageVersion,
    },
    Range {
        from: LanguageVersion,
        till: LanguageVersion,
    },
}

impl LanguageVersionSpecifier {
    pub const fn from(from: LanguageVersion) -> Self {
        Self::From { from }
    }

    pub const fn till(till: LanguageVersion) -> Self {
        Self::Till { till }
    }

    pub const fn range(from: LanguageVersion, till: LanguageVersion) -> Self {
        Self::Range { from, till }
    }

    pub const fn contains(&self, other: LanguageVersion) -> bool {
        let other = other as u32;
        match self {
            Self::From { from } => other >= *from as u32,
            Self::Till { till } => other < *till as u32,
            Self::Range { from, till } => other >= *from as u32 && other < *till as u32,
        }
    }
}

#[cfg(test)]
mod tests;
