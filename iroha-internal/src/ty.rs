use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

pub trait Tokenizable<'a>: ToTokens {
    type ValueType;
    fn type_name(&self) -> TokenStream;

    fn value_token_stream(&self) -> TokenStream;

    fn from_value(value: &'a Self::ValueType) -> Self;
}

pub struct TokenizableVec<'a, T: ToTokens>(pub &'a Vec<T>);

impl<'a, T: ToTokens> Tokenizable<'a> for TokenizableVec<'a, T> {
    type ValueType = Vec<T>;
    fn type_name(&self) -> TokenStream {
        quote::quote! {
            iroha::TokenizableVec
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let value = self.0;
        quote::quote! {
            vec![#(#value),*]
        }
    }

    fn from_value(value: &'a Self::ValueType) -> Self {
        TokenizableVec(value)
    }
}

impl<T: ToTokens> ToTokens for TokenizableVec<'_, T> {
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

pub struct TokenizableString<'a>(pub &'a String);

impl <'a> Tokenizable<'a> for TokenizableString<'a> {
    type ValueType = String;
    fn type_name(&self) -> TokenStream {
        quote::quote! {
            iroha::TokenizableString
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let value = self.0;
        quote::quote! {
            String::from(#value)
        }
    }

    fn from_value(value: &'a Self::ValueType) -> Self {
        TokenizableString(value)
    }
}

impl ToTokens for TokenizableString<'_> {
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
