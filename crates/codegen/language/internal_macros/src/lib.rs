mod spanned_types;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_spanned_types(config: TokenStream, input: TokenStream) -> TokenStream {
    return match spanned_types::derive(config.into(), input.into()) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    };
}
