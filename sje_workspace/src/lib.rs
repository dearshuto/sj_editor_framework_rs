mod document;
mod document_info;
mod observable;
mod project;
mod workspace;
pub use document::Document;
pub use document_info::DocumentInfo;
pub use workspace::Workspace;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[derive(sje_generator_macro::Immutable, Default, Clone, Copy)]
    struct Float2 {
        #[allow(dead_code)]
        pub x: f32,
        #[allow(dead_code)]
        pub y: f32,
    }

    #[derive(sje_generator_macro::Immutable, Default)]
    struct TestData {
        #[allow(dead_code)]
        int_value: i32,
        #[allow(dead_code)]
        float_value: f32,
        #[allow(dead_code)]
        uint_value: u32,
        #[allow(dead_code)]
        struct_value: Float2,
    }

    #[test]
    fn generate_test() {
        // let test_data = Arc::new(TestData::default());
        // let new_instance = test_data.with_float_value(0.1).with_uint_value(77);
        // let new_new_instance = test_data.with_float_value(0.1).with_uint_value(5);
        // assert_eq!(test_data.float_value, Default::default());
        // assert_eq!(new_instance.float_value, 0.1);
        // assert_eq!(new_instance.uint_value, 77);
        // assert_eq!(new_new_instance.uint_value, 5);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
