use syn::{PathArguments, AngleBracketedGenericArguments, Error};
use std::error::Error as StdError;
use syn::export::fmt::{Display, Result as FmtResult, Formatter};
use quote::ToTokens;
use proc_macro2::TokenStream;

pub fn assert_angle_args(arguments: &PathArguments) -> Result<Option<&AngleBracketedGenericArguments>, Error> {
    match arguments {
        PathArguments::None => Ok(None),
        PathArguments::AngleBracketed(result) => Ok(Some(result)),
        _ => Err(Error::new_spanned(
            arguments, "Path argument must be angle bracketed args"
        ))
    }
}

#[derive(Clone, Debug)]
pub struct TokenizableError;

impl StdError for TokenizableError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
            None
    }
}

impl Display for TokenizableError {
     fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
         unreachable!()
     }
}

impl ToTokens for TokenizableError {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unreachable!()
    }
}