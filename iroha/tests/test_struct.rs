use iroha::ToTokens;
use quote::ToTokens;

#[derive(ToTokens)]
struct TestUnit;

#[derive(ToTokens)]
struct TestTuple(i32, i64, Vec<u8>);

#[derive(ToTokens)]
#[Iroha(mod_path="test")]
struct TestStruct {
    a: i32,
    b: i64,
    c: Vec<i64>,
    d: String,
    e: Vec<String>,
}

fn get_result<T: ToTokens>(value: T) -> String {
    let tokens = quote::quote! {
        #value
    };

    tokens.to_string().chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn test_unit() {
    let unit = TestUnit;
    assert_eq!(get_result(&unit), "{TestUnit::new()}");
    let tuple = TestTuple(1, -1, vec![1, 2, 3]);
    assert_eq!(
        get_result(&tuple),
        "{TestTuple::new(1i32,-1i64,{vec![1u8,2u8,3u8]})}"
    );
}

#[test]
fn test_struct() {
    let st = TestStruct {
        a: 0,
        b: 0,
        c: vec![1, 2, 3],
        d: String::from("23333"),
        e: vec![String::from("a"), String::from("b"), String::from("c")],
    };
    assert_eq!(
        get_result(&st),
        "{test::TestStruct::new(0i32,0i64,{vec![1i64,2i64,3i64]},{String::from(\"23333\")},{vec![{String::from(\"a\")},{String::from(\"b\")},{String::from(\"c\")}]})}"
    );
}