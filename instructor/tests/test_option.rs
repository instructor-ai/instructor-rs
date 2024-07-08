extern crate instruct_macros;
extern crate instruct_macros_types;

use instruct_macros::InstructMacro;
use instruct_macros_types::{Parameter, ParameterInfo, StructInfo};
use instructor_ai::from_openai;
use openai_api_rs::v1::api::Client;

#[cfg(test)]
mod tests {
    use std::env;

    use openai_api_rs::v1::{
        chat_completion::{self, ChatCompletionRequest},
        common::GPT4_O,
    };
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test_simple_option() {
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        let instructor_client = from_openai(client);

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct MaybeUser {
            #[description(
                "This is an optional name of a person. If no user name can be found, the field will be null"
            )]
            name: Option<String>,
            error_message: String,
        }

        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from("It's a beautiful day out")),
                name: None,
            }],
        );

        let result = instructor_client
            .chat_completion::<MaybeUser>(req, 3)
            .unwrap();

        assert!(result.name.is_none());
    }

    #[test]
    fn test_complex_option() {
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        let instructor_client = from_openai(client);

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct UserInfo {
            name: String,
            age: u8,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct MaybeUser {
            #[description("This is an optional user field. If the user is not present, the field will be null")]
            user: Option<UserInfo>,
            error_message: String,
        }

        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from("It's a beautiful day out")),
                name: None,
            }],
        );

        let result = instructor_client
            .chat_completion::<MaybeUser>(req, 3)
            .unwrap();

        assert!(result.user.is_none());
    }
}
