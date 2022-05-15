use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod immutable_macro;
mod project_macro;
use immutable_macro::generate_member_functions;
use project_macro::generate_instances;

// 構造体の各 field に with_xx() 関数を自動生成するマクロ
#[proc_macro_derive(Immutable)]
pub fn immutable_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_member_functions(input) {
        Ok(generated) => generated,
        Err(error) => error.to_compile_error().into(),
    }
}

// 構造体の各 field に with_xx() 関数を自動生成するマクロ
#[proc_macro_derive(Workspace)]
pub fn workspace_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_instances(input) {
        Ok(generated) => generated,
        Err(error) => error.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
