use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

extern crate proc_macro;

#[proc_macro_derive(Surrealize)]
pub fn wrap(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the identifier of the original struct
    let struct_name = input.ident;

    // Generate the name of the wrapped struct
    let wrapped_struct_name =
        syn::Ident::new(&format!("SurrealDB{}", struct_name), struct_name.span());

    // Match the data to ensure it's a struct with named fields
    let Data::Struct(data_struct) = input.data else {
        panic!("Surrealize can only be derived for structs.");
    };
    let Fields::Named(fields_named) = data_struct.fields else {
        panic!("Surrealize can only be derived for structs with named fields.");
    };

    let wrapped_fields = {
        let wrapped_fields = fields_named.named.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            let Some(name) = &field.ident else {
                return quote! {
                        #field_name: #field_type
                };
            };
            if name != "id" {
                return quote! {
                        #field_name: #field_type
                };
            }
            quote! {
                id: surrealdb::sql::Thing
            }
        });
        quote! { #(#wrapped_fields,)* }
    };

    let passed_values = {
        let passed_values = fields_named.named.iter().map(|field| {
            let field_name = &field.ident;
            // let field_type = &field.ty;
            let Some(name) = &field.ident else {
                return quote! {
                        #field_name: wrapped.#field_name
                };
            };
            if name != "id" {
                return quote! {
                        #field_name: wrapped.#field_name
                };
            }
            quote! {
                id: *id
            }
        });
        quote! { #(#passed_values,)* }
    };

    // Generate the output code
    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct #wrapped_struct_name {
            #wrapped_fields
        }

        impl From<#wrapped_struct_name> for #struct_name {
            fn from(wrapped: #wrapped_struct_name) -> Self {
                match wrapped.id.id {
                    surrealdb::sql::Id::Uuid(id) => Self {
                        #passed_values
                    },
                    _ => panic!("Unsupported id type."),
                }
            }
        }
    };

    // Convert into a TokenStream and return
    TokenStream::from(expanded)
}
