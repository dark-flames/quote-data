use iroha::ToTokens;
use quote::{quote, ToTokens};

#[derive(ToTokens)]
#[Iroha(mod_path="test")]
enum Test {
    A,
    B,
    C
}

#[derive(ToTokens)]
enum Test2 {
    A,
    B,
    C
}

fn get_string<T: ToTokens>(value: T) -> String {
    let tokens = quote! {
        #value
    };

    tokens.to_string().chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
pub fn test_enum_with_path() {
    let a = get_string(Test::A);
    assert_eq!(a, "{test::Test::A}");
    let b = get_string(Test::B);
    assert_eq!(b, "{test::Test::B}");
    let c = get_string(Test::C);
    assert_eq!(c, "{test::Test::C}");
}

pub fn test_enum() {
    let a = get_string(Test2::A);
    assert_eq!(a, "{Test2::A}");
    let b = get_string(Test2::B);
    assert_eq!(b, "{Test2::B}");
    let c = get_string(Test2::C);
    assert_eq!(c, "{Test2::C}");
}