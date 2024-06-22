use instruct_macros::InstructMacro;
use serde::{Deserialize, Serialize};
use std::env; // Import the macro

mod instructor;
mod parse; // Import the trait module

use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT3_5_TURBO_0613,
};
use parse::{ParameterInfo, StructInfo};

#[derive(InstructMacro, Deserialize, Serialize, Debug)]
/// This is a model which represents a single individual user
struct UserInfo {
    /// This is the name of the user
    name: String,
    /// This is the age of the user
    age: u8,
    /// This is the city of the user
    city: String,
}

impl instructor::InstructMacro for UserInfo {
    fn get_info() -> StructInfo {
        UserInfo::get_info()
    }
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
