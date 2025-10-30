# Breaking Changes

> The following changes have been done to the language grammar/manifest, to adopt the new v2 lexer/parser system:

## Serde

- Using tagged enums for all model types (`#[serde(tag = "type")]`).

## Topics

- Make `Topic::lexical_context` required, instead of creating a "default" context behind the scenes.
- Added `switch_lexical_context_on_reduce` field to `StructItem` and `KeywordItem` to support context switching in the new lexer/parser system. These are the only two grammar items that can trigger context switches for now, but we can add it to more items later if needed.

## Scanners

- Removed `Scanner::TrailingContext` as the new lexer has no backtracking, and tries to scan the longest match by default. Terminals now have a priority (to resolve ambiguities), defined by their order of declaration in the grammar. Later definitions like `MultilineComment` (`///`) have a higher priority than earlier definitions like `SingleLineComment` (`//`).
