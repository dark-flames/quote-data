use proc_macro2::{TokenStream, Span};
use quote::{ToTokens, TokenStreamExt};
use syn::{AngleBracketedGenericArguments, GenericArgument, Error};
use crate::{get_wrapped_value, get_wrapper};

pub trait Tokenizable: ToTokens {
    type ValueType;
    fn type_name(&self) -> TokenStream;

    fn value_token_stream(&self) -> TokenStream;

    fn from_value(value: Self::ValueType) -> Self;
}

pub struct TokenizableVec<T: ToTokens>(pub Vec<T>);

impl<T: ToTokens> Tokenizable for TokenizableVec<T> {
    type ValueType = Vec<T>;
    fn type_name(&self) -> TokenStream {
        quote::quote! {
            iroha::TokenizableVec
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let value = &self.0;
        quote::quote! {
            vec![#(#value),*]
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableVec(value)
    }
}

pub fn vec_convert_token_stream(
    arguments: Option<&AngleBracketedGenericArguments>,
    value_path: &TokenStream
) -> Result<TokenStream, Error> {
    if arguments.is_none() {
        return Err(Error::new(
            Span::call_site(),
            "Vec must have one generic param at least."
        ))
    }
    let nested_type = arguments.unwrap().args.iter().filter_map(
        |arg| {
            match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None
            }
        }
    ).find(|_| true).ok_or_else(
        || Error::new_spanned(
            &arguments.unwrap(),
            "Vec must have one generic param at least."
        )
    )?;

    let wrapped_value = get_wrapped_value(nested_type, quote::quote! {
            item
        }, false, true)?;
    let wrapped_type = get_wrapper(nested_type);
    Ok(quote::quote! {
        iroha::TokenizableVec::from_value(#value_path.iter().map(
            |item| #wrapped_value
        ).collect::<Vec<#wrapped_type>>())
    })
}

impl<T: ToTokens> ToTokens for TokenizableVec<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        tokens.append(proc_macro2::Group::new(
            proc_macro2::Delimiter::Brace,
            quote::quote! {
                #value
            }
        ))
    }
}

pub struct TokenizableString(pub String);

impl Tokenizable for TokenizableString {
    type ValueType = String;
    fn type_name(&self) -> TokenStream {
        quote::quote! {
            iroha::TokenizableString
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let value = &self.0;
        quote::quote! {
            String::from(#value)
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableString(value)
    }
}

pub fn string_convert_token_stream(
    arguments: Option<&AngleBracketedGenericArguments>,
    value_path: &TokenStream
) -> Result<TokenStream, Error> {
    if arguments.is_some() {
        return Err(Error::new_spanned(arguments.unwrap(), "String do not support generic"))
    }
    Ok(quote::quote! {
        iroha::TokenizableString::from_value(#value_path.clone())
    })
}

impl ToTokens for TokenizableString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        tokens.append(proc_macro2::Group::new(
            proc_macro2::Delimiter::Brace,
            quote::quote! {
                #value
            }
        ))
    }
}
