# quote-it
A tokenization Library for Rust.

## Usage
`quote-it` provide derive macro `quote_it::QuoteIt`, 
which implements `quote::ToTokens` for struct or enum.

```rust
use quote_it::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(ToTokens)]
struct Foo {
    a: i32,
    b: i64
}

#[derive(QuoteIt)]
#[mod_path="path::to::mod"]
enum Bar {
    A(u8, String),
    B
}

fn some_fn() -> TokenStream {
    let foo = Foo {a: 1, b: 2};
    let bar = Bar::A(1, "test".to_string);

    quote! {
        || (#foo, #bar)
    }
}
```

## Supported Types
* Any types implemented `quote::ToTokens`
* `String`
* `Vec`, `HashMap`, `HashSet`
* `Result`, `Option`
* `Tuple`
* `std::marker::PhantomData`
