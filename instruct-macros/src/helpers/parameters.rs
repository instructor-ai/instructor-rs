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

pub fn extract_parameters(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            let field_comment = extract_field_description(field);

            quote! {
                parameters.push(ParameterInfo {
                    name: stringify!(#field_name).to_string(),
                    r#type: stringify!(#field_type).to_string(),
                    comment: #field_comment.to_string(),
                });
            }
        })
        .collect()
}
