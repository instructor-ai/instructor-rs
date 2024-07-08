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

            FieldInfo {
                name: field_name
                    .as_ref()
                    .map_or_else(|| "Unnamed".to_string(), |ident| ident.to_string()),
                description: field_comment,
                r#type: serialized_field_type.clone(),
                is_complex: is_complex_type(serialized_field_type.clone()),
            }
        })
        .collect()
}

pub fn is_complex_type(field_type: String) -> bool {
    let simple_types = vec![
        "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "str", "String",
        "u8", "u16", "u32", "u64", "u128", "usize",
    ];

    if simple_types.contains(&field_type.as_str()) {
        return false;
    }

    if field_type.starts_with("Vec<") {
        let inner_type = &field_type[4..field_type.len() - 1];
        return simple_types.contains(&inner_type);
    }

    true
}

pub fn extract_parameters(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    extract_parameter_information(fields)
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let field_type = &field.r#type;
            let field_comment = &field.description;

            if !field.is_complex {
                quote! {
                    parameters.push(Parameter::Field(ParameterInfo {
                        name: #field_name.to_string(),
                        r#type: #field_type.to_string(),
                        comment: #field_comment.to_string(),
                    }));
                }
            } else {
                let field_type = Ident::new(&field.r#type, proc_macro2::Span::call_site()); // Convert string to an identifier
                quote! {
                    parameters.push(#field_type::get_info().wrap_info(#field_name.to_string()));
                }
            }
        })
        .collect()
}
