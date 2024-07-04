extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

#[proc_macro_derive(InstructMacro, attributes(validate))]
pub fn instruct_validate_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`
    let name = &input.ident;

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
                .find(|attr| attr.path.is_ident("validate"))
                .map(|attr| {
                    let meta = attr.parse_meta().expect("Unable to parse attribute");
                    parse_validation_attribute(field_name, &meta)
                })
        })
        .collect();

    // Extract struct-level comment
    let struct_comment = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path.is_ident("doc") {
                if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                    if let Lit::Str(lit) = meta.lit {
                        return Some(lit.value());
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
                        if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                            if let Lit::Str(lit) = meta.lit {
                                return Some(lit.value());
                            }
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
        impl instruct_macros_types::InstructMacro for #name {
            fn get_info() -> instruct_macros_types::StructInfo {
                let mut parameters = Vec::new();
                #(#parameters)*

                StructInfo {
                    name: stringify!(#name).to_string(),
                    description: #struct_comment.to_string(),
                    parameters,
                }
            }

            fn validate(&self) -> Result<(), String> {
                #(#validation_fields)*
                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
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
    let Meta::List(list) = meta else { panic!("Unsupported meta") };
    
    list.nested.iter().map(|nm| {
        let NestedMeta::Meta(Meta::NameValue(nv)) = nm else { panic!("Unsupported nested attribute") };
        let ident = &nv.path;
        let lit = &nv.lit;
        
        match ident.get_ident().unwrap().to_string().as_str() {
            "custom" => {
                let Lit::Str(s) = lit else { panic!("Custom validator must be a string literal") };
                let func = format_ident!("{}", s.value());
                quote! {
                    if let Err(e) = #func(&self.#field_name) {
                        return Err(format!("Validation failed for field '{}': {}", stringify!(#field_name), e));
                    }
                }
            },
            _ => panic!("Unsupported validation type"),
        }
    }).collect()
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