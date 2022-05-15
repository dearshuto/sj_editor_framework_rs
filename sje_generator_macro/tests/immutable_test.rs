use std::sync::Arc;

use sje_generator_macro::Immutable;

#[derive(Immutable)]
struct StructData {
    pub int_value: i32,
    pub uint_value: u32,
}

#[derive(Immutable)]
struct TestData {
    pub struct_data: Arc<StructData>,
}

#[derive(Immutable)]
struct GenericData<T: Clone> {
    generic_value: T,
    arc_value: Arc<StructData>,
}

#[test]
fn simple_test() {
    let struct_data = StructData {
        int_value: 12,
        uint_value: 24,
    };
    let new_int_value_struct_data = struct_data.with_int_value(7);
    let new_uint_value_struct_data = new_int_value_struct_data.with_uint_value(23);

    assert_eq!(new_int_value_struct_data.int_value, 7);
    assert_eq!(new_uint_value_struct_data.int_value, 7);
    assert_eq!(new_uint_value_struct_data.uint_value, 23);
}

#[test]
fn with_struct_test() {
    let struct_data = StructData {
        int_value: 7,
        uint_value: 11,
    };
    let new_struct_data = struct_data.with_int_value(33);
    let test_data = TestData {
        struct_data: Arc::new(struct_data),
    };

    // with 関数を使った更新
    let new_test_data = test_data.with_struct_data(new_struct_data);
    assert_eq!(new_test_data.struct_data.int_value, 33);

    // update 関数を使った更新
    let new_test_data = test_data.update_struct_data(|x| x.with_int_value(109));
    assert_eq!(new_test_data.struct_data.int_value, 109);
}

#[test]
fn with_generic_struc_test() {
    let struct_data = StructData {
        int_value: 100,
        uint_value: 200,
    };
    let generic_data = GenericData {
        generic_value: 7,
        arc_value: Arc::new(struct_data),
    };
    let new_generic_data = generic_data.with_generic_value(9);

    assert_eq!(generic_data.generic_value, 7);
    assert_eq!(new_generic_data.generic_value, 9);
    assert_eq!(new_generic_data.arc_value.int_value, 100);

    let new_generic_data = generic_data.update_arc_value(|x| x.update_int_value(|_| 33));
    assert_eq!(new_generic_data.arc_value.int_value, 33);
}
