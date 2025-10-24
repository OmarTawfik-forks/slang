# Breaking Changes

> The following changes have been done to the language grammar/manifest, to adopt the new v2 lexer/parser system:

## Serde

- Using tagged enums for all model types (`#[serde(tag = "type")]`).

## Scanners

- Removed `Scanner::TrailingContext` as the new lexer has no backtracking, and tries to scan the longest match by default.
- Removed `enabled` field from `TokenDefinition` and `KeywordDefinition` and `FragmentItem`, as the new lexer is now version-agnostic.
- Lexemes now have a priority (to resolve ambiguities), defined by their order of declaration in the grammar. Later definitions like `MultilineComment` (`///`) have a higher priority than earlier definitions like `SingleLineComment` (`//`).
