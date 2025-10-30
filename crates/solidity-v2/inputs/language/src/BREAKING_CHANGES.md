# Breaking Changes

> The following changes have been done to the language grammar/manifest, to adopt the new v2 lexer/parser system:

## Scanners

- Removed `Scanner::TrailingContext` as the new lexer has no backtracking, and tries to scan the longest match by default. Terminals now have a priority (to resolve ambiguities), defined by their order of declaration in the grammar. Later definitions like `MultilineComment` (`///`) have a higher priority than earlier definitions like `SingleLineComment` (`//`).
