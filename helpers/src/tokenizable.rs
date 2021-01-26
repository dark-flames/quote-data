use quote::ToTokens;
use syn::{Type, Error};
use proc_macro2::TokenStream;
use crate::helper::{assert_angle_args, get_nested_types};
use crate::error::IrohaError;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use crate::TokenizableError;

pub fn get_value_wrapper(ty: &Type, value_path: TokenStream, as_ref: bool, clone: bool) -> Result<TokenStream, Error> {
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

    let handlers = [
        TokenizableVec::<String>::convert_token_stream,
        TokenizableString::convert_token_stream,
        TokenizableOption::<String>::convert_token_stream,
        TokenizableResult::<String, TokenizableError>::convert_token_stream,
        TokenizableHashMap::<String, String>::convert_token_stream,
        TokenizableHashSet::<String>::convert_token_stream
    ];

    let result = handlers.iter().fold(
        Ok(None),
        |prev, handler| {
            if let Ok(None) = prev {
                handler(ty, &value_path)
            } else {
                prev
            }
        }
    )?;

    Ok(result.unwrap_or_else(|| quote::quote! {
        #ref_token#value_path#clone_token
    }))
}

pub trait Tokenizable: ToTokens + Clone + Sized {
    type ValueType;

    fn value_token_stream(&self) -> TokenStream;

    fn from_value(value: Self::ValueType) -> Self;

    fn convert_token_stream(
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error>;
}


#[derive(Clone)]
pub struct TokenizableVec<T: ToTokens + Clone>(pub Vec<T>);

impl<T: ToTokens + Clone> Tokenizable for TokenizableVec<T> {
    type ValueType = Vec<T>;

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
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "Vec" {
                return Ok(None)
            }

            let arguments = assert_angle_args(&last_segment.arguments)?;
            let nested_types = get_nested_types(arguments);
            let nested_type = match nested_types.first() {
                Some(r) => r,
                None => return Err(IrohaError::TypeParamCountError("Vec", 1, 0).into_syn_error(ty))
            };

            let wrapped_value = get_value_wrapper(
                nested_type,
                quote::quote! {
                item
            },
                false,
                true,
            )?;

            Ok(Some(quote::quote! {
                iroha::TokenizableVec::from_value(#value_path.iter().map(
                    |item| #wrapped_value
                ).collect())
            }))
        } else {
            Ok(None)
        }
    }
}

impl<T: ToTokens + Clone> ToTokens for TokenizableVec<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        }).to_tokens(tokens)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TokenizableString(pub String);

impl Tokenizable for TokenizableString {
    type ValueType = String;

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
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "String" {
                return Ok(None)
            }

            Ok(Some(quote::quote! {
                iroha::TokenizableString::from_value(#value_path.clone())
            }))
        } else {
            Ok(None)
        }
    }
}

impl ToTokens for TokenizableString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        }).to_tokens(tokens)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TokenizableOption<T: ToTokens + Clone>(pub Option<T>);

impl<T: ToTokens + Clone> Tokenizable for TokenizableOption<T> {
    type ValueType = Option<T>;

    fn value_token_stream(&self) -> TokenStream {
        match &self.0 {
            Some(nested) => quote::quote! {
                Some(#nested)
            },
            None => quote::quote! {
                None
            },
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableOption(value)
    }

    fn convert_token_stream(
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "Option" {
                return Ok(None)
            }

            let arguments = assert_angle_args(&last_segment.arguments)?;
            let nested_types = get_nested_types(arguments);
            let nested_type = match nested_types.first() {
                Some(r) => r,
                None => return Err(IrohaError::TypeParamCountError("Option", 1, 0).into_syn_error(ty))
            };

            let wrapped_value = get_value_wrapper(
                nested_type,
                quote::quote! {
                option_value
            },
                false,
                true,
            )?;

            Ok(Some(quote::quote! {
                iroha::TokenizableOption::from_value(#value_path.as_ref().map(|option_value| #wrapped_value))
            }))
        } else {
            Ok(None)
        }
    }
}

impl<T: ToTokens + Clone> ToTokens for TokenizableOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        }).to_tokens(tokens)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TokenizableResult<T: ToTokens + Clone, E: ToTokens + Clone + std::error::Error>(
    pub Result<T, E>,
);

impl<T, E> Tokenizable for TokenizableResult<T, E>
    where
        T: ToTokens + Clone,
        E: ToTokens + Clone + std::error::Error,
{
    type ValueType = Result<T, E>;

    fn value_token_stream(&self) -> TokenStream {
        match &self.0 {
            Ok(v) => quote::quote! {
                Ok(#v)
            },
            Err(e) => quote::quote! {
                Err(#e)
            },
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableResult(value)
    }

    fn convert_token_stream(
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "Result" {
                return Ok(None)
            }

            let arguments = assert_angle_args(&last_segment.arguments)?;
            let nested_types = get_nested_types(arguments);

            let first_param = nested_types.get(0).ok_or_else(|| {
                IrohaError::TypeParamCountError("Result", 2, 0).into_syn_error(ty)
            })?;

            let second_param = nested_types.get(1).ok_or_else(|| {
                IrohaError::TypeParamCountError("Result", 2, 1).into_syn_error(ty)
            })?;

            let first_wrapped_value = get_value_wrapper(
                first_param,
                quote::quote! {
                result
            },
                false,
                true,
            )?;

            let second_wrapped_value = get_value_wrapper(
                second_param,
                quote::quote! {
                error
            },
                false,
                true,
            )?;

            Ok(Some(quote::quote! {
                iroha::TokenizableResult::from_value(
                    #value_path.clone()
                    .map(|result| #first_wrapped_value)
                    .map_err(|error| #second_wrapped_value)
                )
            }))
        } else {
            Ok(None)
        }
    }
}

impl<T, E> ToTokens for TokenizableResult<T, E>
    where
        T: ToTokens + Clone,
        E: ToTokens + Clone + std::error::Error,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        }).to_tokens(tokens)
    }
}

#[derive(Clone)]
pub struct TokenizableHashMap<K: Eq + Hash + Clone + ToTokens, V: Clone + ToTokens>(
    pub HashMap<K, V>,
);

impl<K, V> Tokenizable for TokenizableHashMap<K, V>
    where
        K: Eq + Hash + Clone + ToTokens,
        V: Clone + ToTokens,
{
    type ValueType = Vec<(K, V)>;

    fn value_token_stream(&self) -> TokenStream {
        let pairs: Vec<TokenStream> = self
            .0
            .iter()
            .map(|(key, value)| {
                quote::quote! {
                    (#key, #value)
                }
            })
            .collect();

        quote::quote! {
            vec![#(#pairs),*].into_iter().collect()
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableHashMap(value.into_iter().collect())
    }

    fn convert_token_stream(
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "HashMap" {
                return Ok(None)
            }

            let arguments = assert_angle_args(&last_segment.arguments)?;
            let nested_types = get_nested_types(arguments);

            let first_param = nested_types.get(0).ok_or_else(|| {
                IrohaError::TypeParamCountError("HashMap", 2, 0).into_syn_error(ty)
            })?;

            let second_param = nested_types.get(1).ok_or_else(|| {
                IrohaError::TypeParamCountError("HashMap", 2, 1).into_syn_error(ty)
            })?;

            let first_wrapped_value = get_value_wrapper(
                first_param,
                quote::quote! {
                key
            },
                false,
                true,
            )?;

            let second_wrapped_value = get_value_wrapper(
                second_param,
                quote::quote! {
                value
            },
                false,
                true,
            )?;

            Ok(Some(quote::quote! {
                iroha::TokenizableHashMap::from_value(
                    #value_path.iter().map(
                        |(key, value)| (#first_wrapped_value, #second_wrapped_value)
                    ).collect()
                )
            }))
        } else {
            Ok(None)
        }
    }
}

impl<K, V> ToTokens for TokenizableHashMap<K, V>
    where
        K: Eq + Hash + Clone + ToTokens,
        V: Clone + ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        })
            .to_tokens(tokens)
    }
}

#[derive(Clone)]
pub struct TokenizableHashSet<T: ToTokens + Clone + Eq + Hash>(pub HashSet<T>);

impl<T> Tokenizable for TokenizableHashSet<T>
    where
        T: ToTokens + Clone + Eq + Hash,
{
    type ValueType = Vec<T>;

    fn value_token_stream(&self) -> TokenStream {
        let items = self.0.iter();
        quote::quote! {
            vec![#(#items),*].into_iter().collect()
        }
    }

    fn from_value(value: Self::ValueType) -> Self {
        TokenizableHashSet(value.into_iter().collect())
    }

    fn convert_token_stream(
        ty: &Type,
        value_path: &TokenStream,
    ) -> Result<Option<TokenStream>, Error> {
        if let Type::Path(type_path) = ty {
            let last_segment = type_path.path.segments.iter().rev().next().unwrap();

            if last_segment.ident != "HashSet" {
                return Ok(None)
            }

            let arguments = assert_angle_args(&last_segment.arguments)?;
            let nested_types = get_nested_types(arguments);
            let nested_type = nested_types.first().ok_or_else(|| {
                IrohaError::TypeParamCountError("HashSet", 1, 0).into_syn_error(ty)
            })?;

            let wrapped_value = get_value_wrapper(
                nested_type,
                quote::quote! {
                item
            },
                false,
                true,
            )?;

            Ok(Some(quote::quote! {
                iroha::TokenizableHashSet::from_value(#value_path.iter().map(
                    |item| #wrapped_value
                ).collect())
            }))
        } else {
            Ok(None)
        }
    }
}

impl<T> ToTokens for TokenizableHashSet<T>
    where
        T: ToTokens + Clone + Eq + Hash,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value_token_stream();
        (quote::quote! {
                #value
        })
            .to_tokens(tokens)
    }
}