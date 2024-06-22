extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

#[proc_macro_derive(InstructMacro, attributes(doc))]
pub fn struct_info_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`
    let name = &input.ident;

    // Extract struct-level comment
    let struct_comment = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path.is_ident("doc") {
                match attr.parse_meta().ok()? {
                    Meta::NameValue(meta) => {
                        if let Lit::Str(lit) = meta.lit {
                            return Some(lit.value());
                        }
                    }
                    _ => {}
                }
            }
            None
        })
        .collect::<Vec<String>>()
        .join(" ");

    // Process each field in the struct
    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
        } else {
            panic!("Unnamed fields are not supported");
        }
    } else {
        panic!("Only structs are supported");
    };

    let parameters: Vec<_> = fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            // Extract field-level comment
            let field_comment = field
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path.is_ident("doc") {
                        match attr.parse_meta().ok()? {
                            Meta::NameValue(meta) => {
                                if let Lit::Str(lit) = meta.lit {
                                    return Some(lit.value());
                                }
                            }
                            _ => {}
                        }
                    }
                    None
                })
                .collect::<Vec<String>>()
                .join(" ");

            quote! {
                parameters.push(ParameterInfo {
                    name: stringify!(#field_name).to_string(),
                    r#type: stringify!(#field_type).to_string(),
                    comment: #field_comment.to_string(),
                });
            }
        })
        .collect();

    let expanded = quote! {
        impl instructor::InstructMacro for #name {
            fn get_info() -> parse::StructInfo {
                let mut parameters = Vec::new();
                #(#parameters)*

                StructInfo {
                    name: stringify!(#name).to_string(),
                    description: #struct_comment.to_string(),
                    parameters,
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
