use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, DeriveInput};

// 構造体の各 field に with_xx() 関数を自動生成するマクロ
#[proc_macro_derive(Immutable)]
pub fn immutable_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_member_functions(input) {
        Ok(generated) => generated,
        Err(error) => error.to_compile_error().into(),
    }
}

fn generate_member_functions(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    // 構造体のみを対象にする
    // 他のデータ型に指定された場合はエラー
    let struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    // 生成するメンバ関数
    let mut fields = Vec::new();
    for field in &struct_data.fields {
        // 構造体を初期化するコード
        // fn with_value(&self, value) -> Self {
        //     Self {
        //        x: self.x, // <- self の値を引き継ぐ部分を生成
        //        y: self.y  // <-
        //        z: value
        //    }
        // }
        //
        let init_fields = struct_data
            .fields
            .iter()
            .filter(|x| *x != field)
            .map(|x| {
                let name_info = x.ident.as_ref().unwrap();
                quote!(#name_info: self.#name_info,)
            })
            .collect::<Vec<_>>();

        let name_info = field.ident.as_ref().unwrap();
        let type_info = &field.ty;

        // with_XX 形式で関数を生やす
        let generated_method_name: proc_macro2::TokenStream =
            format!("with_{}", name_info.unraw().to_string())
                .parse()
                .unwrap();

        // 関数本体の生成
        // fn with_value(&self, value) -> Arc<Self> {}
        fields.push(quote! {
            pub fn #generated_method_name(&self, value: #type_info) -> Arc<Self> {
                let new_instance = Self {
                    #name_info: value,
                    #(#init_fields)*
                };
                Arc::new(new_instance)
            }
        });
    }

    // struct名の情報
    let struct_name = &derive_input.ident;
    // generics, where句の情報
    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    // impl struct Xyz {
    //   // #(#fields)* で ↓ の fields の内容がリピートで出力される
    //   pub fn with_<field>(&self, value: type) -> Arc<Self> {
    //       let new_instance = Self{ };
    //       Arc::new(new_instance)
    //   }
    // }
    let expanded = quote! {
        impl #impl_generics #struct_name #where_clause {
            #(#fields)*
        }
    };

    Ok(expanded.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
