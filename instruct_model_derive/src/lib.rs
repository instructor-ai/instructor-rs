use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(InstructModel)]
pub fn derive_instruct_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        #[async_trait]
        impl InstructModel for #name {
            async fn extract_from_response(response: ChatCompletionResponse) -> Result<Self> {
                let content = response
                    .choices
                    .first()
                    .ok_or_else(|| anyhow!("No choices in response"))?
                    .message
                    .content
                    .clone();
                serde_json::from_str(&content).map_err(|e| anyhow!("Failed to parse {}: {}", stringify!(#name), e))
            }
        }
    };
    TokenStream::from(expanded)
}
