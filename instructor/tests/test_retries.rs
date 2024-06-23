extern crate instruct_macros;
extern crate instruct_macros_types;

use instruct_macros::InstructMacro;
use instruct_macros_types::{ParameterInfo, StructInfo};
use instructor_ai::from_openai;
use openai_api_rs::v1::api::Client;

use serde::{de, Deserializer};

#[cfg(test)]
mod tests {
    use std::env;

    use openai_api_rs::v1::{
        chat_completion::{self, ChatCompletionRequest},
        common::GPT3_5_TURBO,
    };
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test_from_openai() {
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        let instructor_client = from_openai(client);

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        // This represents a single user
        struct UserInfo {
            // This represents the name of the user
            #[serde(deserialize_with = "validate_uppercase")]
            name: String,
            // This represents the age of the user
            age: u8,
        }

        fn validate_uppercase<'de, D>(de: D) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(de)?;
            println!("{}", s);
            if s.chars().any(|c| c.is_lowercase()) {
                return Err(de::Error::custom(format!(
                    "Name '{}' should be entirely in uppercase. Examples: 'TIMOTHY', 'JANE SMITH'",
                    s
                )));
            }
            Ok(s.to_uppercase())
        }

        let req = ChatCompletionRequest::new(
            GPT3_5_TURBO.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(
                    "John Doe is a 30 year old software engineer",
                )),
                name: None,
            }],
        );

        let result = instructor_client
            .chat_completion::<UserInfo>(req, 3)
            .unwrap();
        assert_eq!(result.age, 30);
        assert_eq!(result.name, "JOHN DOE");
    }
}
