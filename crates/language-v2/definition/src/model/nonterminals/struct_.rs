use indexmap::IndexMap;
use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Field, FieldsErrorRecovery, Identifier, VersionSpecifier};

/// A `StructItem` is a nonterminal that can have fields.
/// It roughly corresponds to a sequence of `Item`s.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct StructItem {
    pub name: Identifier,

    /// If set, this struct is a gateway to the specified lexical context.
    /// The first field must be in the struct's own (topic) context,
    /// while remaining fields must reference items from the declared context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_context: Option<Identifier>,

    /// Whether the struct is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// Error recovery information if this struct supports it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_recovery: Option<FieldsErrorRecovery>,

    /// The fields of the struct, in the order they should appear in the source code
    #[serde(with = "indexmap::map::serde_seq")]
    pub fields: IndexMap<Identifier, Field>,
}
