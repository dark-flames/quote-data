use iroha::ToTokens;
use quote::quote;

#[derive(ToTokens)]
enum Test {
    A,
    B,
    C
}

fn get_string(value: Test) -> String {
    let tokens = quote! {
        #value
    };

    tokens.to_string().chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
pub fn test_enum() {
    let a = get_string(Test::A);
    assert_eq!(a, "{Test::A}");
    let b = get_string(Test::B);
    assert_eq!(b, "{Test::B}");
    let c = get_string(Test::C);
    assert_eq!(c, "{Test::C}");
}