use core::panic;
use instruct_macros::StructName;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, Tool};
use openai_api_rs::v1::common::GPT3_5_TURBO_0613;
use serde::{Deserialize, Serialize};
use std::{env, vec};

#[derive(StructName, Deserialize, Serialize, Debug)]
// This is a model which represents a single individual user
struct UserInfo {
    // This is the name of the user
    name: String,
    // This is the age of the user
    age: u8,
}

trait StructName {
    fn struct_name() -> &'static str;
    fn field_types() -> &'static [&'static str];
}

fn main() {
    fn generate_function_call<T: StructName>() -> Tool {
        let func = chat_completion::Function {
            name: String::from(T::struct_name()),
            description: Some(String::from("This is a User")),
            parameters: chat_completion::FunctionParameters {
                schema_type: chat_completion::JSONSchemaType::Object,
                properties: {
                    let field_types = T::field_types();
                    let mut properties_map = std::collections::HashMap::new();
                    for field in field_types.iter() {
                        let parts: Vec<&str> = field.splitn(2, ':').map(|s| s.trim()).collect();
                        let (name, ty) = (parts[0], parts[1]);
                        let schema_type = match ty {
                            "String.ty" => chat_completion::JSONSchemaType::String,
                            "u8.ty" => chat_completion::JSONSchemaType::Number,
                            _ => panic!("Invalid type: {}", ty),
                        };

                        properties_map.insert(
                            name.to_string(),
                            Box::new(chat_completion::JSONSchemaDefine {
                                schema_type: Some(schema_type),
                                description: Some(format!(
                                    "This is a {} property that belongs to the object",
                                    name
                                )),
                                ..Default::default()
                            }),
                        );
                    }
                    Some(properties_map)
                },
                required: Some(T::field_names().iter().map(|&s| s.to_string()).collect()),
            },
        };
        chat_completion::Tool {
            r#type: chat_completion::ToolType::Function,
            function: func,
        }
    }

    let function_call = generate_function_call::<UserInfo>();

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO_0613.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(
                "John doe is 30 and lives in new york?",
            )),
            name: None,
        }],
    )
    .tools(vec![function_call]);

    let result = client.chat_completion(req);

    match &result.unwrap().choices[0].message.tool_calls {
        Some(tool_calls) => {
            let user = tool_calls[0].function.arguments.clone().unwrap();
            let user_info: UserInfo =
                serde_json::from_str(&user).expect("Failed to parse user info");

            println!("Name: {}", user_info.name);
            println!("Age: {}", user_info.age);
        }
        _ => {
            println!("No tool calls");
        }
    }
}
