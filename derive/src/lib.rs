extern crate proc_macro;

mod r#enum;
mod helper;
mod r#struct;

use helper::MOD_PATH;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use r#enum::EnumStructure;
use r#struct::StructStructure;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, Error, Expr, Lit, Meta};

#[proc_macro_derive(QuoteIt, attributes(mod_path))]
pub fn derive_to_tokens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input! {input as DeriveInput};
    let mut mod_path_tokens: Result<Option<TokenStream2>, Error> = Ok(None);

    for attr in &input.attrs {
        if attr.path() == MOD_PATH {
            match &attr.meta {
                Meta::NameValue(mod_path) => {
                    mod_path_tokens = match match &mod_path.value {
                        Expr::Lit(path_lit) => match &path_lit.lit {
                            Lit::Str(path_str) => Ok(path_str.value()),
                            _ => Err(Error::new_spanned(
                                &mod_path,
                                "`mod_path` must be a string",
                            ))
                        },
                        _ => Err(Error::new_spanned(
                            &mod_path,
                            "`mod_path` must be a string",
                        )),
                    }
                        .map(|path| {
                            TokenStream2::from_str(path.as_str()).map_err(|_| {
                                Error::new_spanned(
                                    &mod_path,
                                    "Value of `mod_path` must be a path of mod",
                                )
                            })
                        }) {
                        Ok(Ok(tokens)) => Ok(Some(tokens)),
                        Ok(Err(e)) => Err(e),
                        Err(e) => Err(e),
                    }
                }
                _ => {
                    mod_path_tokens = Err(Error::new_spanned(
                        attr,
                        "Attribute `mod_path` must be a named value.",
                    ))
                }
            }
        }
    }

    let mod_path_tokens = match mod_path_tokens {
        Ok(result) => result,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    let result = match &input.data {
        Data::Enum(_) => {
            EnumStructure::from_ast(&input, mod_path_tokens).map(|s| s.get_implement())
        }
        Data::Struct(_) => {
            StructStructure::from_ast(&input, mod_path_tokens).map(|s| s.get_implement())
        }
        _ => Err(Error::new_spanned(&input, "Unknown data type")),
    };

    TokenStream::from(match result {
        Ok(Ok(tokens)) => tokens,
        Ok(Err(e)) => e.to_compile_error(),
        Err(e) => e.to_compile_error(),
    })
}
