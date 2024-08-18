use crate::helper::Interpolated;
use crate::r#struct::StructField;
use heck::ToSnakeCase;
use helpers::get_wrapped_value;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Token, Data, DeriveInput, Error, Fields, Ident, Variant as SynVariant, Generics, GenericParam};
use syn::punctuated::Punctuated;

pub struct EnumStructure {
    name: Ident,
    variants: Vec<Variant>,
    generics: Generics,
    mod_path: Option<TokenStream>,
}

impl EnumStructure {
    pub fn from_ast(input: &DeriveInput, mod_path: Option<TokenStream>) -> Result<Self, Error> {
        let name = input.ident.clone();

        let data_enum = match &input.data {
            Data::Enum(data) => data,
            _ => unreachable!(),
        };

        let variants = data_enum
            .variants
            .iter()
            .map(Variant::from_ast)
            .collect::<Result<Vec<Variant>, Error>>()?;

        let generics = input.generics.clone();

        Ok(EnumStructure {
            name,
            variants,
            generics,
            mod_path
        })
    }

    pub fn get_implement(&self) -> Result<TokenStream, Error> {
        let name = &self.name;
        let variants: Vec<TokenStream> = self
            .variants
            .iter()
            .map(|variant| variant.arm_token_stream(&self.name, &self.mod_path))
            .collect::<Result<Vec<_>, _>>()?;

        let generics = &self.generics.params;
        let generics_without_bounds: Punctuated<GenericParam, Token![,]> = self.generics.params.clone()
            .iter()
            .map(
                |item| {
                    if let GenericParam::Type(param) = item {
                        let mut new_param = param.clone();
                        new_param.attrs = vec![];
                        new_param.colon_token = None;
                        new_param.bounds = Punctuated::default();
                        new_param.eq_token = None;
                        new_param.default = None;

                        GenericParam::Type(new_param)
                    } else {
                        item.clone()
                    }
                }
            ).collect();
        let where_clause = &self.generics.where_clause;

        Ok(quote! {
            impl<#generics> quote::ToTokens for #name <#generics_without_bounds> #where_clause {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    use quote_it::Tokenizable;
                    match self {
                            #(#variants),*
                    }.to_tokens(tokens);
                }
            }
        })
    }
}

struct Variant {
    name: Ident,
    named: bool,
    fields: Vec<StructField>,
}

impl Variant {
    pub fn from_ast(variant: &SynVariant) -> Result<Self, Error> {
        let name = variant.ident.clone();
        let named = matches!(variant.fields, Fields::Named(_));
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| StructField::from_ast(field, index))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Variant {
            name,
            named,
            fields,
        })
    }

    pub fn arm_token_stream(
        &self,
        enum_ident: &Ident,
        mod_path: &Option<TokenStream>,
    ) -> Result<TokenStream, Error> {
        let name = &self.name;

        let mut arguments = vec![];
        let mut temp_values = vec![];
        let mut construct_params = vec![];

        for field in self.fields.iter() {
            let argument = format_ident!(
                "{}_{}",
                enum_ident.to_string().to_snake_case(),
                field.name()
            );
            let temp_value_ident = format_ident!(
                "temp_{}_{}",
                enum_ident.to_string().to_snake_case(),
                field.name()
            );

            let value = get_wrapped_value(field.ty(), argument.to_token_stream(), false, false)?;

            let interpolated_param = Interpolated(temp_value_ident.to_string());

            arguments.push(if let Some(name) = field.ident() {
                quote::quote! {
                    #name: #argument
                }
            } else {
                argument.to_token_stream()
            });

            temp_values.push(quote::quote! {
                let #temp_value_ident = #value
            });
            construct_params.push(if let Some(name) = field.ident() {
                quote::quote! {
                    #name: #interpolated_param
                }
            } else {
                quote::quote! {
                    #interpolated_param
                }
            })
        }

        let arguments_tokens = if !self.fields.is_empty() {
            if self.named {
                quote! {
                    {#(#arguments),*}
                }
            } else {
                quote! {
                    (#(#arguments),*)
                }
            }
        } else {
            TokenStream::new()
        };

        let mod_path_token = mod_path
            .as_ref()
            .map(|path| quote! {#path::})
            .unwrap_or_default();

        let construct_token_stream = if self.named {
            quote::quote! {
                {#(#construct_params),*}
            }
        } else if self.fields.is_empty() {
            TokenStream::new()
        } else {
            quote::quote! {
                (#(#construct_params),*)
            }
        };

        Ok(quote! {
            #enum_ident::#name#arguments_tokens => {
                #(#temp_values;)*
                quote::quote! {
                    #mod_path_token#enum_ident::#name#construct_token_stream
                }
            }
        })
    }
}
