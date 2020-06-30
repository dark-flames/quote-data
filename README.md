# Iroha
Iroha is a tokenization Library for Rust.

## Usage
Iroha provide derive macro `iroha::ToTokens`.
Derived struct or enum will be implemented `quote::ToTokens`.

```rust
use iroha::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(ToTokens)]
struct Foo {
    a: i32,
    b: i64
}

#[derive(ToTokens)]
enum Bar {
    A,
    B
}

fn some_fn() -> TokenStream {
    let foo = Foo {a: 1, b: 2};
    let bar = Bar::A;

    quote! {
        || (#foo, #bar)
    }
}
```

## Supported Type 
* Any types witch implemented `quote::ToTokens`
* `String`
* `Vec`, `HashMap`, `HashSet`
* `Result`, `Option`