use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;

trait TokenizableCollection {
    type VecType;
    fn to_tokens(&self) -> TokenStream;

    fn from_vec(value: Vec<Self::VecType>) -> Self;
}

trait CollectionOfTokenizableCollection {
    type VecType;
    fn to_tokens(&self) -> TokenStream;

    fn from_vec(value: Vec<Vec<Self::VecType>>) -> Self;
}

impl<T: ToTokens> TokenizableCollection for Vec<T> {
    type VecType = T;

    fn to_tokens(&self) -> TokenStream {
        let values = self;
        quote! {
            vec![#(#values,)*]
        }
    }

    fn from_vec(value: Vec<Self::VecType>) -> Self {
        value
    }
}

impl <D: TokenizableCollection> CollectionOfTokenizableCollection for Vec<D> {
    type VecType = D::VecType;

    fn to_tokens(&self) -> TokenStream {
        let values: Vec<TokenStream> = self.iter().map(
            |value| value.to_tokens()
        ).collect();
        quote! {
            vec![#(#values,)*]
        }
    }

    fn from_vec(value: Vec<Vec<Self::VecType>>) -> Self {
        value.into_iter().map(
            |item| D::from_vec(item)
        ).collect()
    }
}