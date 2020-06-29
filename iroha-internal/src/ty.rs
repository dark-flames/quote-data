use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{AngleBracketedGenericArguments, Error, PathArguments};
use crate::{get_wrapped_value, get_wrapper};
use std::clone::Clone;
use std::collections::HashMap;
use std::hash::Hash;
use crate::helper::get_nested_types;

pub trait Tokenizable: ToTokens {
    type ValueType;
    fn type_name(argument: &PathArguments) -> TokenStream;

    fn value_token_stream(&self) -> TokenStream;

    fn from_value(value: Self::ValueType) -> Self;

    fn convert_token_stream(
        arguments: Option<&AngleBracketedGenericArguments>,
        value_path: &TokenStream
    ) -> Result<TokenStream, Error>;
}

#[derive(Clone)]
pub struct TokenizableVec<T: ToTokens + Clone>(pub Vec<T>);

impl<T: ToTokens + Clone> Tokenizable for TokenizableVec<T> {
    type ValueType = Vec<T>;
    fn type_name(argument: &PathArguments) -> TokenStream {
        quote::quote! {
            iroha::TokenizableVec#argument
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

    fn convert_token_stream(
        arguments: Option<&AngleBracketedGenericArguments>,
        value_path: &TokenStream
    ) -> Result<TokenStream, Error> {
        let nested_types = get_nested_types(arguments);
        let nested_type = nested_types.first().ok_or_else(
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
}

impl<T: ToTokens + Clone> ToTokens for TokenizableVec<T> {
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

#[derive(Clone)]
pub struct TokenizableString(pub String);

impl Tokenizable for TokenizableString {
    type ValueType = String;
    fn type_name(_argument: &PathArguments) -> TokenStream {
        quote::quote! {
            iroha::TokenizableString
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let value = &self.0;
        quote::quote! {
            #value.to_string()
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableString(value)
    }

    fn convert_token_stream(
        arguments: Option<&AngleBracketedGenericArguments>,
        value_path: &TokenStream
    ) -> Result<TokenStream, Error> {
        if let Some(args) = arguments {
            return Err(Error::new_spanned(args, "String do not support generic"))
        }
        Ok(quote::quote! {
            iroha::TokenizableString::from_value(#value_path.clone())
        })
    }
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

#[derive(Clone)]
pub struct TokenizableOption<T: ToTokens + Clone>(pub Option<T>);

impl <T: ToTokens + Clone> Tokenizable for TokenizableOption<T> {
    type ValueType = Option<T>;

    fn type_name(argument: &PathArguments) -> TokenStream {
        quote::quote! {
            iroha::TokenizableOption#argument
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        match &self.0 {
            Some(nested) => quote::quote! {
                Some(#nested)
            },
            None => quote::quote! {
                None
            }
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableOption(value)
    }

    fn convert_token_stream(arguments: Option<&AngleBracketedGenericArguments>, value_path: &TokenStream) -> Result<TokenStream, Error> {
        let nested_type = get_nested_types(arguments);
        let nested_type = nested_type.first().ok_or_else(
            || Error::new_spanned(
                &arguments.unwrap(),
                "Option must have one generic param at least."
            )
        )?;

        let wrapped_value = get_wrapped_value(nested_type, quote::quote! {
            option_value
        }, false, true)?;
        Ok(quote::quote! {
            iroha::TokenizableOption::from_value(#value_path.as_ref().map(|option_value| #wrapped_value))
        })
    }
}

impl<T: ToTokens + Clone> ToTokens for TokenizableOption<T> {
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

pub struct TokenizableResult<
    T: ToTokens + Clone,
    E: ToTokens + Clone + std::error::Error
>(pub Result<T, E>);

impl <
    T: ToTokens + Clone,
    E: ToTokens + Clone + std::error::Error
> Tokenizable for TokenizableResult<T, E> {
    type ValueType = Result<T, E>;

    fn type_name(argument: &PathArguments) -> TokenStream {
        quote::quote! {
            iroha::TokenizableResult#argument
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        match &self.0 {
            Ok(v) => quote::quote! {
                Ok(#v)
            },
            Err(e) => quote::quote! {
                Err(#e)
            }
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableResult(value)
    }

    fn convert_token_stream(arguments: Option<&AngleBracketedGenericArguments>, value_path: &TokenStream) -> Result<TokenStream, Error> {
        let nested_types = get_nested_types(arguments);

        let first_param = nested_types.get(0).ok_or_else(
            || Error::new_spanned(arguments.unwrap(), "Result must have two generic params.")
        )?;

        let second_param = nested_types.get(1).ok_or_else(
            || Error::new_spanned(arguments.unwrap(), "Result must have two generic params.")
        )?;

        let first_wrapped_value = get_wrapped_value(
            first_param, quote::quote! {
                result
            }, false, true
        )?;

        let second_wrapped_value = get_wrapped_value(
            second_param, quote::quote! {
                error
            }, false, true
        )?;

        Ok(quote::quote! {
            iroha::TokenizableResult::from_value(
                #value_path.clone()
                .map(|result| #first_wrapped_value)
                .map_err(|error| #second_wrapped_value)
            )
        })
    }
}

impl<
    T: ToTokens + Clone,
    E: ToTokens + Clone + std::error::Error
> ToTokens for TokenizableResult<T, E> {
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

pub struct TokenizableHashMap<
    K: Eq + Hash + Clone + ToTokens,
    V: Clone + ToTokens
>(pub HashMap<K, V>);

impl<K, V> Tokenizable for TokenizableHashMap<K, V>
    where
        K: Eq + Hash + Clone + ToTokens,
        V: Clone + ToTokens
{
    type ValueType = Vec<(K, V)>;

    fn type_name(argument: &PathArguments) -> TokenStream {
        quote::quote! {
            iroha::TokenizableHashMap#argument
        }
    }

    fn value_token_stream(&self) -> TokenStream {
        let pairs: Vec<TokenStream> = self.0.iter().map(
            |(key, value)| quote::quote! {
                (#key, #value)
            }
        ).collect();

        quote::quote! {
            vec![#(#pairs),*].into_iter().collect()
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableHashMap(value.into_iter().collect())
    }

    fn convert_token_stream(arguments: Option<&AngleBracketedGenericArguments>, value_path: &TokenStream) -> Result<TokenStream, Error> {
        let nested_types = get_nested_types(arguments);

        let first_param = nested_types.get(0).ok_or_else(
            || Error::new_spanned(arguments.unwrap(), "Result must have two generic params.")
        )?;

        let second_param = nested_types.get(1).ok_or_else(
            || Error::new_spanned(arguments.unwrap(), "Result must have two generic params.")
        )?;

        let first_wrapped_value = get_wrapped_value(
            first_param, quote::quote! {
                key
            }, false, true
        )?;

        let second_wrapped_value = get_wrapped_value(
            second_param, quote::quote! {
                value
            }, false, true
        )?;

        Ok(quote::quote! {
            iroha::TokenizableHashMap::from_value(
                #value_path.iter().map(
                    |(key, value)| (#first_wrapped_value, #second_wrapped_value)
                ).collect()
            )
        })
    }
}

impl<
    K: Eq + Hash + Clone + ToTokens,
    V: Clone + ToTokens
> ToTokens for TokenizableHashMap<K, V> {
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
