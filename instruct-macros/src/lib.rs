extern crate proc_macro;

mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields};

/// Derive macro for InstructMacro trait.
///
/// This macro generates the implementation for extracting struct/enum metadata
/// for LLM function calling. For validation, use the `validator` crate's
/// `#[derive(Validate)]` along with this macro.
///
/// # Example
/// ```rust,ignore
/// use instruct_macros::InstructMacro;
/// use validator::Validate;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(InstructMacro, Validate, Serialize, Deserialize)]
/// struct UserInfo {
///     #[validate(length(min = 1))]
///     name: String,
///     #[validate(range(min = 0, max = 150))]
///     age: u8,
/// }
/// ```
#[proc_macro_derive(InstructMacro, attributes(description))]
pub fn instruct_validate_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = match &input.data {
        Data::Struct(_) => generate_instruct_macro_struct(&input),
        Data::Enum(_) => generate_instruct_macro_enum(&input),
        _ => panic!("InstructMacro can only be derived for structs and enums"),
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn generate_instruct_macro_enum(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("Only enums are supported"),
    };

    let enum_variants: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();

    // Extract struct-level comment
    let description = extract_attribute_value(&input.attrs, "description");

    let enum_info = quote! {
        instruct_macros_types::InstructMacroResult::Enum(instruct_macros_types::EnumInfo {
            title: stringify!(#name).to_string(),
            r#enum: vec![#(#enum_variants.to_string()),*],
            r#type: stringify!(#name).to_string(),
            description: #description.to_string(),
            is_optional: false,
            is_list: false
        })
    };

    quote! {
        // Enums don't need field validation, implement empty Validate
        impl ::validator::Validate for #name {
            fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
                Ok(())
            }
        }

        impl instruct_macros_types::InstructMacro for #name {
            fn get_info() -> instruct_macros_types::InstructMacroResult {
                #enum_info
            }
        }
    }
}

fn extract_attribute_value(attrs: &[Attribute], attr_name: &str) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident(attr_name) {
                attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn generate_instruct_macro_struct(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let description = extract_attribute_value(&input.attrs, "description");

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

    let parameters = helpers::extract_parameters(fields);

    let expanded = quote! {
        impl instruct_macros_types::InstructMacro for #name {
            fn get_info() -> instruct_macros_types::InstructMacroResult {
                let mut parameters = Vec::new();
                #(#parameters)*

                instruct_macros_types::InstructMacroResult::Struct(StructInfo {
                    name: stringify!(#name).to_string(),
                    description: #description.to_string(),
                    parameters,
                    is_optional: false,
                    is_list: false,
                })
            }
        }
    };

    expanded
}
