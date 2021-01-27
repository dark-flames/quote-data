use super::helper::Interpolated;
use helpers::get_wrapped_value;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Error, Field as SynField, Fields, Ident, Index, Type, Generics, GenericParam};
use syn::punctuated::Punctuated;
use syn::Token;

enum StructType {
    NoField,
    Tuple,
    Struct,
}

impl StructType {
    pub fn get_params(&self, values: TokenStream) -> TokenStream {
        match self {
            StructType::NoField => TokenStream::new(),
            StructType::Tuple => quote::quote! {(#values)},
            StructType::Struct => quote::quote! {{#values}},
        }
    }
}

#[allow(dead_code)]
pub struct StructStructure {
    name: Ident,
    fields: Option<Vec<StructField>>,
    generics: Generics,
    mod_path: Option<TokenStream>,
    struct_type: StructType,
}

impl StructStructure {
    pub fn from_ast(input: &DeriveInput, mod_path: Option<TokenStream>) -> Result<Self, Error> {
        let name = input.ident.clone();

        let data_struct = match &input.data {
            Data::Struct(data) => data,
            _ => unreachable!(),
        };

        let fields = if let Fields::Unit = &data_struct.fields {
            None
        } else {
            Some(
                data_struct
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| StructField::from_ast(field, index))
                    .collect::<Result<Vec<StructField>, Error>>()?,
            )
        };

        let struct_type = match &data_struct.fields {
            Fields::Unit => StructType::NoField,
            Fields::Unnamed(_) => StructType::Tuple,
            Fields::Named(_) => StructType::Struct,
        };

        let generics = input.generics.clone();

        Ok(StructStructure {
            name,
            fields,
            generics,
            mod_path,
            struct_type,
        })
    }

    pub fn get_implement(self) -> Result<TokenStream, Error> {
        let name = &self.name;
        let (field_idents, fn_new_params, temp_values) = match &self.fields {
            Some(fields_vec) => (
                fields_vec
                    .iter()
                    .map(|field| field.get_temp_value_ident())
                    .collect::<Vec<Ident>>(),
                fields_vec
                    .iter()
                    .map(|field| field.get_construct_param())
                    .collect::<Vec<TokenStream>>(),
                fields_vec
                    .iter()
                    .map(|field| field.temp_value_token_stream())
                    .collect::<Result<Vec<TokenStream>, Error>>()?,
            ),
            _ => (Vec::new(), Vec::new(), Vec::new()),
        };

        let params = self
            .struct_type
            .get_params(quote::quote! {#(#field_idents,)*});

        let construct_params: Vec<Interpolated> = field_idents
            .iter()
            .map(|ident| Interpolated(ident.to_string()))
            .collect();

        let mod_path_token = self
            .mod_path
            .as_ref()
            .map(|path| quote::quote! {#path::})
            .unwrap_or_default();

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

        Ok(quote::quote! {
            impl<#generics> #name <#generics_without_bounds> #where_clause {
                pub fn new(#(#fn_new_params),*) -> Self {
                    #name #params
                }
            }

            impl<#generics> quote::ToTokens for #name <#generics_without_bounds> #where_clause {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    use iroha::Tokenizable;
                    #(#temp_values;)*

                    (quote::quote! {
                        #mod_path_token#name::new(#(#construct_params),*)
                    }).to_tokens(tokens);
                }
            }
        })
    }
}

#[allow(dead_code)]
pub(crate) struct StructField {
    ident: Option<Ident>,
    index: usize,
    ty: Type,
}

impl StructField {
    pub fn from_ast(field: &SynField, index: usize) -> Result<Self, Error> {
        let name = field.ident.clone();
        let ty = field.ty.clone();

        Ok(StructField {
            ident: name,
            index,
            ty,
        })
    }

    pub fn name(&self) -> String {
        self.ident.clone()
            .map(|ident| ident.to_string())
            .unwrap_or_else(|| self.index.to_string())
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn ident(&self) -> Option<Ident> {
        self.ident.clone()
    }

    fn get_ident(&self) -> TokenStream {
        if let Some(ident) = &self.ident {
            quote::quote! {
                self.#ident
            }
        } else {
            let index = Index::from(self.index);
            quote::quote! {
                self.#index
            }
        }
    }

    pub fn get_construct_param(&self) -> TokenStream {
        let value = self.get_temp_value_ident();
        let ty = &self.ty;

        quote::quote! {
            #value: #ty
        }
    }

    pub fn get_temp_value_ident(&self) -> Ident {
        if let Some(ident) = &self.ident {
            ident.clone()
        } else {
            quote::format_ident!("field_{}", self.index)
        }
    }

    pub fn temp_value_token_stream(&self) -> Result<TokenStream, Error> {
        let temp_value_ident = self.get_temp_value_ident();
        let value = get_wrapped_value(&self.ty, self.get_ident(), true, false)?;
        Ok(quote::quote! {
            let #temp_value_ident = #value
        })
    }
}
