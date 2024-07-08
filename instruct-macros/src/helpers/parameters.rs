use instruct_macros_types::FieldInfo;
use quote::quote;
use syn::{Expr, Ident, Lit};

fn extract_field_description(field: &syn::Field) -> String {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("description") {
                let meta = attr.parse_args().expect("Unable to parse attribute");
                if let Expr::Lit(expr_lit) = &meta {
                    if let Lit::Str(lit) = &expr_lit.lit {
                        return Some(lit.value());
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn extract_parameter_information(fields: &syn::FieldsNamed) -> Vec<FieldInfo> {
    fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_comment = extract_field_description(field);

            let field_type = &field.ty;
            let serialized_field_type = quote!(#field_type).to_string();

            let serialized_field_type = if serialized_field_type.contains("Option <")
                || serialized_field_type.contains("Vec <")
            {
                serialized_field_type.replace(" ", "")
            } else {
                serialized_field_type
            };

            FieldInfo {
                name: field_name
                    .as_ref()
                    .map_or_else(|| "Unnamed".to_string(), |ident| ident.to_string()),
                description: field_comment,
                r#type: serialized_field_type.clone(),
                is_complex: is_complex_type(serialized_field_type.clone()),
                is_optional: is_option_type(&serialized_field_type),
                is_list: is_list_type(serialized_field_type.clone()),
            }
        })
        .collect()
}

fn is_list_type(field_type: String) -> bool {
    field_type.starts_with("Vec<") && field_type.ends_with(">")
}

pub fn is_complex_type(field_type: String) -> bool {
    let simple_types = vec![
        "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "str", "String",
        "u8", "u16", "u32", "u64", "u128", "usize",
    ];

    let option_types: Vec<String> = simple_types
        .iter()
        .map(|&t| format!("Option<{}>", t))
        .collect();

    let vec_types: Vec<String> = simple_types
        .iter()
        .map(|&t| format!("Vec<{}>", t))
        .collect();

    if simple_types.contains(&field_type.as_str()) {
        return false;
    }

    if option_types.contains(&field_type) {
        return false;
    }

    if vec_types.contains(&field_type) {
        return false;
    }

    true
}

fn is_option_type(field_type: &str) -> bool {
    field_type.starts_with("Option<") && field_type.ends_with(">")
}

fn extract_nested_type(field_type: &str) -> String {
    if is_option_type(field_type) {
        field_type[7..field_type.len() - 1].to_string()
    } else {
        field_type.to_string()
    }
}

pub fn extract_parameters(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    extract_parameter_information(fields)
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let field_type = &field.r#type;
            let field_comment = &field.description;
            let is_option = is_option_type(field_type);
            let is_list = field.is_list;

            

            if !field.is_complex {

                let field_type = if is_list {
                    extract_nested_type(&field_type[4..field_type.len() - 1])
                } else {
                    field_type.to_string()
                };

                quote! {
                    parameters.push(Parameter::Field(ParameterInfo {
                        name: #field_name.to_string(),
                        r#type: #field_type.to_string(),
                        comment: #field_comment.to_string(),
                        is_optional: #is_option,
                        is_list: #is_list,
                    }));
                }
            } else if is_option_type(field_type) {
                let field_type = extract_nested_type(field_type);
                let field_type = Ident::new(&field_type, proc_macro2::Span::call_site()); // Convert string to an identifier

                quote! {
                    parameters.push(#field_type::get_info().override_description(#field_comment.to_string()).set_optional(#is_option).wrap_info(#field_name.to_string()));
                }
            } else {
                let is_list = field.is_list;
                

                let field_type = if is_list {
                    extract_nested_type(&field_type[4..field_type.len() - 1])
                } else {
                    field_type.to_string()
                };

                let field_type = Ident::new(&field_type, proc_macro2::Span::call_site()); // Convert string to an identifier
                
                
                quote! {
                    parameters.push(#field_type::get_info().override_description(#field_comment.to_string()).set_optional(#is_option).set_list(#is_list).wrap_info(#field_name.to_string()));
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_complex_type() {
        // Simple types
        let simple_types = vec![
            "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "str",
            "String", "u8", "u16", "u32", "u64", "u128", "usize",
        ];

        for simple_type in &simple_types {
            assert_eq!(is_complex_type(simple_type.to_string()), false);
            assert_eq!(is_complex_type(format!("Option<{}>", simple_type)), false);
        }

        // Complex types

        assert_eq!(is_complex_type("Option<User>".to_string()), true);
    }

    #[test]
    fn test_extract_nested_type() {
        // Test cases for extract_nested_type function
        let test_cases = vec![("Option<User>", "User")];

        for (input, expected) in test_cases {
            assert_eq!(extract_nested_type(input), expected);
        }
    }
}
