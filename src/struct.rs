use proc_macro::Ident;
use proc_macro2::TokenStream;

#[allow(dead_code)]
pub struct EnumStructure {
    name: Ident,
    fields: Vec<Field>,
    mod_path: Option<TokenStream>
}

#[allow(dead_code)]
struct Field {
    name: Option<Ident>,
}