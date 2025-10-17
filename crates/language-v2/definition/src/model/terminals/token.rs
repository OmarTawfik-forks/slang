use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TokenItem {
    pub name: Identifier,

    pub definitions: Vec<TokenDefinition>,
}

impl TokenItem {
    pub fn is_unique(&self) -> bool {
        self.definitions
            .iter()
            .all(|definition| definition.is_unique())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TokenDefinition {
    pub scanner: Scanner,
}

impl TokenDefinition {
    pub fn is_unique(&self) -> bool {
        self.scanner.is_unique()
    }
}
