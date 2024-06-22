use std::collections::HashMap;

use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest, JSONSchemaDefine},
    error::APIError,
};

use crate::parse::{ParameterInfo, StructInfo};

pub struct InstructorClient {
    client: Client,
}

pub trait InstructMacro {
    fn get_info() -> StructInfo;
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

    pub fn chat_completion<T>(&self, req: ChatCompletionRequest) -> Result<T, APIError>
    where
        T: InstructMacro + for<'de> serde::Deserialize<'de>,
    {
        let parsed_model: StructInfo = T::get_info();

        let properties = Self::get_parameters::<T>(parsed_model.parameters);
        let func_call = chat_completion::Tool {
            r#type: chat_completion::ToolType::Function,
            function: chat_completion::Function {
                name: parsed_model.name,
                description: Some(parsed_model.description),
                parameters: chat_completion::FunctionParameters {
                    schema_type: chat_completion::JSONSchemaType::Object,
                    properties: Some(properties),
                    required: Some(vec![]),
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

                        let res: T = serde_json::from_str(&arguments).unwrap();
                        Ok(res)
                    }
                    _ => panic!("Unexpected number of tool calls"),
                }
            }
            _ => panic!("Unexpected finish reason"),
        }
    }
}

pub fn from_openai(client: Client) -> InstructorClient {
    InstructorClient::new(client)
}
