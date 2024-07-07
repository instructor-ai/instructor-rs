use instruct_macros_types::FieldInfo;
use quote::quote;
use syn::{Expr, Lit};

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
            let field_type = &field.ty;
            let field_comment = extract_field_description(field);

            FieldInfo {
                name: field_name
                    .as_ref()
                    .map_or_else(|| "Unnamed".to_string(), |ident| ident.to_string()),
                description: field_comment,
                r#type: quote!(#field_type).to_string(),
                is_complex: false,
            }
        })
        .collect()
}

pub fn extract_parameters(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    extract_parameter_information(fields)
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let field_type = &field.r#type;
            let field_comment = &field.description;

            quote! {
                parameters.push(ParameterInfo {
                    name: #field_name.to_string(),
                    r#type: #field_type.to_string(),
                    comment: #field_comment.to_string(),
                });
            }
        })
        .collect()
}
