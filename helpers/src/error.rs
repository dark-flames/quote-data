use thiserror::Error;
use syn::Error;
use quote::ToTokens;

#[derive(Error, Debug)]
pub enum QuoteItError {
    #[error("{0} expected {1} generic parameters, {1} got")]
    TypeParamCountError(&'static str, usize, usize),
    #[error("Path argument must be angle bracketed args")]
    NotAngleBracketedArgs
}

impl QuoteItError {
    pub fn into_syn_error(self, span: impl ToTokens) -> Error {
        Error::new_spanned(span, self.to_string())
    }
}