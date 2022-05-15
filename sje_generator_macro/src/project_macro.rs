use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_instances(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    // 構造体のみを対象にする
    // 他のデータ型に指定された場合はエラー
    let _struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    let fields = _struct_data
        .fields
        .iter()
        .map(|x| {
            let name_info = x.ident.as_ref().unwrap();
            let type_info = &x.ty;
            quote!( pub #name_info : std::collections::HashMap<i32, #type_info>, )
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        pub struct Project {
            #(#fields)*
        }

        pub struct Workspace {
            project: Project,
        }
    };

    Ok(expanded.into())
}
