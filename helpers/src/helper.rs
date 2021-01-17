use proc_macro2::TokenStream;
use quote::ToTokens;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use syn::{AngleBracketedGenericArguments, Error, GenericArgument, PathArguments, Type};

pub fn assert_angle_args(
    arguments: &PathArguments,
) -> Result<Option<&AngleBracketedGenericArguments>, Error> {
    match arguments {
        PathArguments::None => Ok(None),
        PathArguments::AngleBracketed(result) => Ok(Some(result)),
        _ => Err(Error::new_spanned(
            arguments,
            "Path argument must be angle bracketed args",
        )),
    }
}

pub fn get_nested_types(arguments: Option<&AngleBracketedGenericArguments>) -> Vec<&Type> {
    match arguments {
        Some(args) => args
            .args
            .iter()
            .filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
            .collect(),
        None => Vec::new(),
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
