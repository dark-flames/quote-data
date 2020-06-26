pub mod ty;

use syn::Type;
use proc_macro2::TokenStream;

pub fn get_wrapped_value(ty: &Type, value_path: TokenStream) -> TokenStream {
    if let Type::Path(type_path) = ty {
        let last_segment = type_path.path.segments.iter().rev().next().unwrap();
        match last_segment.ident.to_string().as_str() {
            "String" => quote::quote! {
                iroha::TokenizableString::from_value(&#value_path)
            },
            "Vec" => quote::quote! {
                iroha::TokenizableVec::from_value(&#value_path)
            },
            _ => quote::quote! {
                &#value_path
            }
        }
    } else {
        value_path
    }
}