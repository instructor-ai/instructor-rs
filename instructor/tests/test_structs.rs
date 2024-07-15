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
    fn test_enum() {
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        let instructor_client = from_openai(client);

        #[derive(InstructMacro, Debug, Serialize, Deserialize, PartialEq)]
        #[description("This is a label representing whether or not an email is spam or not")]
        enum Label {
            Spam,
            NotSpam,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        #[description("This is a struct representing an email classification")]
        struct Email {
            #[description("Reasoning")]
            chain_of_thought: String,
            label: Label,
        }

        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(
                    "I am a nigerian prince coming to ask you for some $$",
                )),
                name: None,
            }],
        );

        let result = instructor_client.chat_completion::<Email>(req, 3).unwrap();

        assert_eq!(result.label, Label::Spam);
    }

    #[test]
    fn test_nested_struct() {
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        let instructor_client = from_openai(client);

        #[derive(InstructMacro, Debug, Serialize, Deserialize, PartialEq)]
        #[description("This is a struct representing an address")]
        struct Address {
            #[description("The street of the address")]
            street: String,
            #[description("The city of the address")]
            city: String,
            #[description("The country of the address")]
            country: String,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize, PartialEq)]
        #[description("This is a struct representing user details")]
        struct UserDetail {
            #[description("The age of the user")]
            age: i32,
            #[description("The name of the user")]
            name: String,
            #[description("The address of the user")]
            address: Address,
            #[description("The security clearance of the user")]
            security_clearance: SecurityClearance,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize, PartialEq)]
        #[description("This is an enum representing the user's security clearance")]
        enum SecurityClearance {
            #[description("Low security clearance")]
            Low,
            #[description("Medium security clearance")]
            Medium,
            #[description("High security clearance")]
            High,
        }

        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(
                    "Extract the user details from the following string: \"John Doe is 30 years old, lives at 123 Main St, Anytown, USA, and has a security clearance of High.\"",
                )),
                name: None,
            }],
        );

        let result = instructor_client
            .chat_completion::<UserDetail>(req, 3)
            .unwrap();

        let expected_user_detail = UserDetail {
            age: 30,
            name: "John Doe".to_string(),
            address: Address {
                street: "123 Main St".to_string(),
                city: "Anytown".to_string(),
                country: "USA".to_string(),
            },
            security_clearance: SecurityClearance::High,
        };

        assert_eq!(result, expected_user_detail);
    }
}
