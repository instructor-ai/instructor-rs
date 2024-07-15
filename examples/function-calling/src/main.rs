use core::fmt;
use std::env;

use instruct_macros::InstructMacro;
use instruct_macros_types::{Parameter, ParameterInfo, StructInfo};
use instructor_ai::from_openai;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT4_O,
};
use serde::{Deserialize, Serialize};

#[derive(InstructMacro, Debug, Serialize, Deserialize)]
enum SearchType {
    Web,
    Image,
    Video,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(InstructMacro, Debug, Serialize, Deserialize)]
struct Search {
    #[description("Topic of the search")]
    topic: String,
    #[description("Query to search for relevant content")]
    query: String,
    #[description("Type of search")]
    stype: SearchType,
}

impl Search {
    fn execute(&self) {
        println!(
            "Executing a(n) {} Search for '{}' from query: '{}'",
            self.stype, self.topic, self.query,
        )
    }
}

fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let instructor_client = from_openai(client);

    let q = "Search for a picture of a cat";

    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(format!(
                "Consider the data below and segment it into a search quer:\n{}",
                q
            ))),
            name: None,
        }],
    );

    let search = instructor_client.chat_completion::<Search>(req, 3).unwrap();

    search.execute()
    /*
    Executing a(n) Image Search for 'cat' from query: 'picture of a cat'
    */
}
