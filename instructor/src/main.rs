use serde::de::{self};
use serde::{Deserialize, Serialize};
use std::env; // Import the macro

mod instructor;

use instruct_macros::InstructMacro; // Ensure this is a derive macro
use instruct_macros_types::{ParameterInfo, StructInfo}; // Import the trait
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT3_5_TURBO_0613,
};

#[derive(InstructMacro, Deserialize, Serialize, Debug)]
/// This is a model which represents a single individual user
struct UserInfo {
    /// This is the name of the user
    #[serde(deserialize_with = "uppercase_name")]
    name: String,
    /// This is the age of the user
    age: u8,
    /// This is the city of the user
    city: String,
}

fn uppercase_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct UppercaseNameVisitor;

    impl<'de> de::Visitor<'de> for UppercaseNameVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an uppercase string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value
                .chars()
                .all(|c| c.is_uppercase() || !c.is_alphabetic())
            {
                Ok(value.to_string())
            } else {
                Err(E::custom(format!(
                    "name should be in uppercase (Eg. JANE vs jane), got '{}'",
                    value
                )))
            }
        }
    }

    deserializer.deserialize_str(UppercaseNameVisitor)
}

fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap());
    let instructor_client = instructor::from_openai(client);

    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO_0613.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(
                "John Doe is 30 years old and lives in New York",
            )),
            name: None,
        }],
    );

    let res = instructor_client.chat_completion::<UserInfo>(req).unwrap();
    println!("{:?}", res);
}
