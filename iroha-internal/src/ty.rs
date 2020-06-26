use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

pub trait Tokenizable<'a>: ToTokens {
    type ValueType;

    fn type_name(&self) -> TokenStream;

    fn value_token_stream(&self) -> TokenStream;

    fn from_value(value: &'a Self::ValueType) -> Self ;

    fn token_stream(&self) -> TokenStream {
        let value = self.value_token_stream();
        let type_name = self.type_name();

        quote::quote! {
            #type_name::from_value(#value)
        }
    }
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
        let value = &self.0;
        quote::quote! {
            vec![#(#value,)*]
        }
    }

    fn from_value(value: &'a Self::ValueType) -> Self {
        TokenizableVec(value)
    }
}

impl<T: ToTokens> ToTokens for TokenizableVec<'_, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(proc_macro2::Group::new(
            proc_macro2::Delimiter::Brace,
            self.token_stream()
        ))
    }
}
