use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::borrow::BorrowMut;
use syn::{fold::Fold, parse_quote, Error, Field, GenericArgument, Item, ItemMod, Result, Type};

pub fn derive(config: TokenStream, input: TokenStream) -> Result<TokenStream> {
    if !config.is_empty() {
        return Err(Error::new_spanned(
            config,
            "Attribute does not support configuration.",
        ));
    }

    let mut input_mod = syn::parse2::<ItemMod>(input)?;

    let input_items = match input_mod.content.borrow_mut() {
        Some((_, input_items)) => input_items,
        None => {
            return Err(Error::new_spanned(
                input_mod,
                "Expected a module containing items within.",
            ));
        }
    };

    let spanned_items = input_items.iter().filter_map(RewriteItem::execute);

    // Use [Item::Verbatim] so that we don't end up parsing the generated items again:
    input_items.push(Item::Verbatim(quote! {
        pub(crate) mod spanned {
            pub use super::*;

            #(#spanned_items)*
        }
    }));

    return Ok(input_mod.into_token_stream());
}

struct RewriteItem;

impl RewriteItem {
    fn execute(input: &Item) -> Option<Item> {
        match &input {
            Item::Enum(_) | Item::Struct(_) => {
                return Some(RewriteItem.fold_item(input.to_owned()));
            }
            Item::Use(_) => {
                return None;
            }
            _ => {
                let error = Error::new_spanned(input, "Only enums and structs are supported.");
                return Some(Item::Verbatim(error.to_compile_error()));
            }
        }
    }
}

impl Fold for RewriteItem {
    fn fold_field(&mut self, input: Field) -> Field {
        return Field {
            ty: WrapType::execute(input.ty),
            ..input
        };
    }
}

struct WrapType {
    has_generic_args: bool,
}

impl WrapType {
    fn execute(input: Type) -> Type {
        let mut instance = WrapType {
            has_generic_args: false,
        };

        let result = instance.fold_type(input);

        return if instance.has_generic_args {
            result
        } else {
            parse_quote! {
                crate::internals::Spanned<#result>
            }
        };
    }
}

impl Fold for WrapType {
    fn fold_generic_argument(&mut self, input: GenericArgument) -> GenericArgument {
        if let GenericArgument::Type(inner) = input {
            self.has_generic_args = true;

            // Proceed to wrap the generic arg:
            return GenericArgument::Type(WrapType::execute(inner));
        } else {
            return syn::fold::fold_generic_argument(self, input);
        }
    }
}
