use instruct_macros_types::InstructMacro;
use std::{collections::HashMap, vec};

use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest, JSONSchemaDefine},
    error::APIError,
};

use instruct_macros_types::{ParameterInfo, StructInfo};

pub struct InstructorClient {
    client: Client,
}

impl InstructorClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn get_parameters<T>(parameters: Vec<ParameterInfo>) -> HashMap<String, Box<JSONSchemaDefine>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut properties = HashMap::new();

        for param in parameters {
            let schema_type = match param.r#type.as_str() {
                "String" => Some(chat_completion::JSONSchemaType::String),
                "u8" => Some(chat_completion::JSONSchemaType::Number),
                _ => None,
            };

            properties.insert(
                param.name.clone(),
                Box::new(chat_completion::JSONSchemaDefine {
                    schema_type,
                    description: Some(param.comment.clone()),
                    ..Default::default()
                }),
            );
        }

        properties
    }

    fn get_required(parameters: Vec<ParameterInfo>) -> Vec<String> {
        parameters.iter().map(|p| p.name.clone()).collect()
    }

    pub fn chat_completion<T>(
        &self,
        req: ChatCompletionRequest,
        max_retries: u8,
    ) -> Result<T, APIError>
    where
        T: InstructMacro + for<'de> serde::Deserialize<'de>,
    {
        let parsed_model: StructInfo = T::get_info();
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
        let properties = Self::get_parameters::<T>(parsed_model.parameters.clone());

        let func_call = chat_completion::Tool {
            r#type: chat_completion::ToolType::Function,
            function: chat_completion::Function {
                name: parsed_model.name,
                description: Some(parsed_model.description),
                parameters: chat_completion::FunctionParameters {
                    schema_type: chat_completion::JSONSchemaType::Object,
                    properties: Some(properties),
                    required: Some(Self::get_required(parsed_model.parameters.clone())),
                },
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
                            format!("Unexpected number of tool calls: {:?}", tool_calls);
                        return Err(serde::de::Error::custom(error_message));
                    }
                }
            }
            _ => panic!("Unexpected finish reason"),
        }
    }
}

pub fn from_openai(client: Client) -> InstructorClient {
    InstructorClient::new(client)
}
