use syn::{Ident, DeriveInput, Error, Data, Variant as SynVariant};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use heck::SnakeCase;

pub struct EnumStructure {
    name: Ident,
    variants: Vec<Variant>,
    mod_path: Option<TokenStream>
}

impl EnumStructure {
    pub fn from_ast(input: &DeriveInput, mod_path: Option<TokenStream>) -> Result<Self, Error> {
        let name = input.ident.clone();

        let data_enum = match &input.data {
            Data::Enum(data) => data,
            _ => unreachable!()
        };

        let variants = data_enum.variants.iter()
            .map(Variant::from_ast).collect::<Result<Vec<Variant>, Error>>()?;

        Ok(EnumStructure {
            name,
            variants,
            mod_path
        })
    }

    pub fn get_implement(&self) -> Result<TokenStream, Error> {
        let name = &self.name;
        let variants: Vec<TokenStream> = self.variants.iter().map(
            |variant| variant.arm_token_stream(&self.name, &self.mod_path)
        ).collect();

        Ok(quote! {
            impl quote::ToTokens for #name {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    use quote::TokenStreamExt;
                    tokens.append(proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        match self {
                            #(#variants),*
                        },
                    ))
                }
            }
        })
    }
}

struct Variant {
    name: Ident,
    argument_count: usize
}

impl Variant {
    pub fn from_ast(variant: &SynVariant) -> Result<Self, Error> {
        let name = variant.ident.clone();
        let argument_count = variant.fields.len();

        Ok(Variant {
            name,
            argument_count
        })
    }

    pub fn arm_token_stream(&self, enum_ident: &Ident, mod_path: &Option<TokenStream>) -> TokenStream {
        let name = &self.name;

        let arguments: Vec<Ident> = (0..self.argument_count).collect::<Vec<_>>().iter().map(
            |index| format_ident!(
                "{}_{}_{}",
                enum_ident.to_string().to_snake_case(),
                name.to_string().to_snake_case(),
                index
            )
        ).collect();

        let arguments_tokens  = match !arguments.is_empty() {
            true => quote! {
                (#(#arguments),*)
            },
            false => TokenStream::new()
        };

        let mod_path_token = mod_path.as_ref().map(
            |path| quote! {#path::}
        ).unwrap_or_default();

        quote! {
            #enum_ident::#name#arguments_tokens => quote::quote! {
                #mod_path_token#enum_ident::#name#arguments_tokens
            }
        }
    }
}

