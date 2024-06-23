use anyhow::Result;
use async_trait::async_trait;
pub use instruct_model_derive::InstructModel;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
// const GPT_4_O: &str = "gpt-4o";

#[async_trait]
pub trait InstructModel: Sized {
    async fn extract_from_response(response: ChatCompletionResponse) -> Result<Self>;
}

pub struct Instructor {
    client: Client,
}

impl Instructor {
    pub fn from_openai(api_key: String) -> Self {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", api_key).parse().unwrap(),
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Instructor { client }
    }

    pub async fn extract<T: InstructModel + DeserializeOwned>(&self, prompt: &str) -> Result<T> {
        let request = ChatCompletionRequest {
            model: GPT_3_5_TURBO.to_string(),
            messages: vec![
                ChatCompletionMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant. Please respond with valid JSON for the shared struct.".to_string(),
                },
                ChatCompletionMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
        };
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await?;
        T::extract_from_response(response).await
    }
}

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}

#[derive(Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatCompletionMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct ChatCompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
