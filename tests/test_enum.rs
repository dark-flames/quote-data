use quote_it::QuoteIt;
use quote::ToTokens;
use std::marker::PhantomData;

#[derive(QuoteIt)]
#[mod_path = "test"]
enum Test {
    A,
    B,
    C,
}

#[derive(QuoteIt)]
enum Test2 {
    A,
    B,
    C,
}

#[derive(QuoteIt)]
enum Test3<T> {
    A(u8, u16, String, Vec<u8>),
    B{a: u8, b: u16, c: String, d: Vec<u8>},
    C(PhantomData<T>)
}

fn get_string<T: ToTokens>(value: T) -> String {
    let tokens = quote::quote! {
        #value
    };

    tokens
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

#[test]
pub fn test_enum_with_path() {
    let a = get_string(Test::A);
    assert_eq!(a, "test::Test::A");
    let b = get_string(Test::B);
    assert_eq!(b, "test::Test::B");
    let c = get_string(Test::C);
    assert_eq!(c, "test::Test::C");
}

#[test]
pub fn test_enum() {
    let a = get_string(Test2::A);
    assert_eq!(a, "Test2::A");
    let b = get_string(Test2::B);
    assert_eq!(b, "Test2::B");
    let c = get_string(Test2::C);
    assert_eq!(c, "Test2::C");
}

#[test]
pub fn test_enum_with_unnamed_field() {
    let a = get_string(Test3::<u8>::A(1, 1, "test".to_string(), vec![1, 2, 3]));
    assert_eq!(a, "Test3::A(1u8,1u16,\"test\".to_string(),vec![1u8,2u8,3u8])");
    let b = get_string(Test3::<u8>::B{
        a: 1, b: 1, c: "test".to_string(),
        d: vec![1, 2, 3]
    });
    assert_eq!(b, "Test3::B{a:1u8,b:1u16,c:\"test\".to_string(),d:vec![1u8,2u8,3u8]}");
    let c = get_string(Test3::<u8>::C(PhantomData::default()));
    assert_eq!(c, "Test3::C(std::marker::PhantomData::default())");
}