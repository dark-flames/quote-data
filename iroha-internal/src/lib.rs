pub mod ty;

use syn::{Type, Error, PathArguments, AngleBracketedGenericArguments};
use proc_macro2::TokenStream;
use ty::*;
use syn::export::ToTokens;

fn assert_angle_args(arguments: &PathArguments) -> Result<Option<&AngleBracketedGenericArguments>, Error> {
    match arguments {
        PathArguments::None => Ok(None),
        PathArguments::AngleBracketed(result) => Ok(Some(result)),
        _ => Err(Error::new_spanned(
            arguments, "Path argument must be angle bracketed args"
        ))
    }
}

pub fn get_wrapped_value(ty: &Type, value_path: TokenStream, as_ref: bool, clone: bool) -> Result<TokenStream, Error> {
    let ref_token = if as_ref {
        quote::quote! {&}
    } else {
        TokenStream::new()
    };

    let clone_token = if clone {
        quote::quote! {.clone()}
    } else {
        TokenStream::new()
    };
    if let Type::Path(type_path) = ty {
        let last_segment = type_path.path.segments.iter().rev().next().unwrap();
        let args = assert_angle_args(&last_segment.arguments)?;
        match last_segment.ident.to_string().as_str() {
            "Vec" => vec_convert_token_stream(args, &value_path),
            "String" => string_convert_token_stream(args, &value_path),
            _ => Ok(quote::quote! {
                #ref_token#value_path#clone_token
            })
        }
    } else {
        Ok(quote::quote! {
            #ref_token#value_path#clone_token
        })
    }
}

pub fn get_wrapper(ty: &Type) -> TokenStream {
    if let Type::Path(type_path) = ty {
        let last_segment = type_path.path.segments.iter().rev().next().unwrap();
        let arguments = &last_segment.arguments;
        match last_segment.ident.to_string().as_str() {
            "Vec" => Some(quote::quote! {
                iroha::TokenizableVec#arguments
            }),
            "String" => Some(quote::quote! {
                iroha::TokenizableString
            }),
            _ => None
        }
    } else {
        None
    }.unwrap_or(ty.to_token_stream())
}