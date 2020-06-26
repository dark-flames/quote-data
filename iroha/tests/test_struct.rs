use iroha::ToTokens;
use quote::ToTokens;

#[derive(ToTokens)]
struct TestUnit;

#[derive(ToTokens)]
struct TestTuple(i32, i64, Vec<u8>);

#[derive(ToTokens)]
struct TestStruct {
    a: i32,
    b: i64,
    c: Vec<i64>
}

fn get_result<T: ToTokens>(object: &T) -> String {
    let tokens = quote::quote! {
        #object
    };

    tokens.to_string().chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn test_unit () {
    let unit = TestUnit;
    assert_eq!(get_result(&unit), "{TestUnit::new()}");
    let tuple = TestTuple(1, -1, vec![1, 2, 3]);
    assert_eq!(get_result(&tuple), "{TestTuple::new(1i32,-1i64,{iroha::TokenizableVec::from_value(vec![1u8,2u8,3u8,])})}");
    let st = TestStruct {
        a: 0,
        b: 0,
        c: vec![1, 2, 3]
    };
    assert_eq!(get_result(&st), "{TestStruct::new(0i32,0i64,{iroha::TokenizableVec::from_value(vec![1i64,2i64,3i64,])})}");
}