mod helper;
mod tokenizable;
mod error;

pub use helper::TokenizableError;

use proc_macro2::TokenStream;
use syn::{Error, Type};
pub use tokenizable::*;

pub fn get_wrapped_value(
    ty: &Type,
    value_path: TokenStream,
    as_ref: bool,
    clone: bool,
) -> Result<TokenStream, Error> {
    get_value_wrapper(ty, value_path, as_ref, clone)
}
