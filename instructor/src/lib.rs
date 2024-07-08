use std::vec;
mod helpers;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    error::APIError,
};

use instruct_macros_types::{InstructMacro, InstructMacroResult, Parameter, StructInfo};

pub struct InstructorClient {
    client: Client,
}

impl InstructorClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn chat_completion<T>(
        &self,
        req: ChatCompletionRequest,
        max_retries: u8,
    ) -> Result<T, APIError>
    where
        T: InstructMacro + for<'de> serde::Deserialize<'de>,
    {
        let parsed_model: StructInfo = match T::get_info() {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let mut error_message: Option<String> = None;

        for _ in 0..max_retries {
            let mut req = req.clone();

            if let Some(ref error) = error_message {
                let new_message = chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::user,
                    content: chat_completion::Content::Text(error.clone()),
                    name: None,
                };
                req.messages.push(new_message);

                println!("Error encountered: {}", error);
            }

            let result = self._retry_sync::<T>(req.clone(), parsed_model.clone());

            match result {
                Ok(value) => {
                    match T::validate(&value) {
                        Ok(_) => {}
                        Err(e) => {
                            error_message =
                                Some(format!("Validation Error: {:?}. Please fix the issue", e));
                            continue;
                        }
                    }

                    return Ok(value);
                }
                Err(e) => {
                    error_message =
                        Some(format!("Validation Error: {:?}. Please fix the issue", e));
                    continue;
                }
            }
        }

        Err(APIError {
            message: format!("Unable to derive model: {:?}", error_message),
        })
    }

    fn _retry_sync<T>(
        &self,
        req: ChatCompletionRequest,
        parsed_model: StructInfo,
    ) -> Result<T, serde_json::Error>
    where
        T: InstructMacro + for<'de> serde::Deserialize<'de>,
    {
        let func_call = chat_completion::Tool {
            r#type: chat_completion::ToolType::Function,
            function: chat_completion::Function {
                name: parsed_model.name.clone(),
                description: Some(parsed_model.description.clone()),
                parameters: helpers::get_response_model(parsed_model.clone()),
            },
        };

        let req = req
            .tools(vec![func_call])
            .tool_choice(chat_completion::ToolChoiceType::Auto);

        let result = self.client.chat_completion(req).unwrap();

        match result.choices[0].finish_reason {
            Some(chat_completion::FinishReason::tool_calls) => {
                // TODO: Support more than one tool at some point?
                let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();

                match tool_calls.len() {
                    1 => {
                        let tool_call = &tool_calls[0];
                        let arguments = tool_call.function.arguments.clone().unwrap();
                        return serde_json::from_str(&arguments);
                    }
                    _ => {
                        // TODO: Support multiple tool calls at some point
                        let error_message =
                            format!("Unexpected number of tool calls: {:?}. PLease only generate a single tool call.", tool_calls);
                        return Err(serde::de::Error::custom(error_message));
                    }
                }
            }
            _ => {
                let error_message =
                    "You must call a tool. Make sure to adhere to the provided response format."
                        .to_string();
                return Err(serde::de::Error::custom(error_message));
            }
        }
    }
}

pub fn from_openai(client: Client) -> InstructorClient {
    InstructorClient::new(client)
}
