extern crate proc_macro;

mod helpers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta};

#[proc_macro_derive(InstructMacro, attributes(validate, description))]
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
        })
    };

    quote! {
        impl instruct_macros_types::InstructMacro for #name {
            fn get_info() -> instruct_macros_types::InstructMacroResult {
                #enum_info
            }


            fn validate(&self) -> Result<(), String> {
                Ok(())
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

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let validation_fields: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_name = &f.ident;
            f.attrs
                .iter()
                .find(|attr| attr.path().is_ident("validate"))
                .map(|attr| {
                    let meta = attr.parse_args().expect("Unable to parse attribute");
                    parse_validation_attribute(field_name, &meta)
                })
        })
        .collect();

    // Extract struct-level comment
    let struct_comment = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Ok(Meta::NameValue(meta)) = attr.parse_args::<Meta>() {
                    if let Expr::Lit(expr_lit) = &meta.value {
                        if let Lit::Str(lit) = &expr_lit.lit {
                            return Some(lit.value());
                        }
                    }
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
                })
            }

            fn validate(&self) -> Result<(), String> {
                #(#validation_fields)*
                Ok(())
            }
        }
    };

    expanded
}

/// Parses the validation attribute and generates corresponding validation code.
///
/// This function processes custom validation attributes, expanding them into function calls
/// that perform the specified validation. It supports custom validators that take a reference
/// to the field type and return a Result with a string error type.
fn parse_validation_attribute(
    field_name: &Option<syn::Ident>,
    meta: &Meta,
) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    match meta {
        Meta::NameValue(name_value) if name_value.path.is_ident("custom") => {
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) = &name_value.value
            {
                let func = syn::Ident::new(&lit_str.value(), proc_macro2::Span::call_site());
                let tokens = quote! {
                    if let Err(e) = #func(&self.#field_name) {
                        return Err(format!("Validation failed for field '{}': {}", stringify!(#field_name), e));
                    }
                };
                output.extend(tokens);
            } else {
                panic!("Custom validator must be a string literal");
            }
        }
        _ => panic!("Unsupported validation attribute"),
    }

    output
}

/// Custom attribute macro for field validation in structs.
///
/// This procedural macro attribute is designed to be applied to structs,
/// enabling custom validation for their fields. When the `validate` method
/// is called on an instance of the decorated struct, it triggers the specified
/// custom validation functions for each annotated field.
#[proc_macro_attribute]
pub fn validate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let syn::ItemFn { sig, block, .. } = input;

    let expanded = quote! {
        #sig {
            #block
        }
    };

    TokenStream::from(expanded)
}
