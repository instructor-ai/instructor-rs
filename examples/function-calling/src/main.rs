use core::fmt;
use std::env;

use instruct_macros::InstructMacro;
use instruct_macros_types::{ParameterInfo, StructInfo};
use instructor_ai::from_openai;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT4_O,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
// This represents a single search
struct Search {
    // Topic of the search
    topic: String,
    // Query to search for relevant content
    query: String,
    // Type of search
    stype: SearchType,
}

impl Search {
    fn execute(&self) {
        println!(
            "Searching for '{}' with query '{}' using '{}'",
            self.topic, self.query, self.stype
        )
    }
}

#[derive(InstructMacro, Debug, Serialize, Deserialize)]
// This represents a list of searches
struct Searches {
    // List of searches
    items: Vec<Search>,
}

fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let instructor_client = from_openai(client);

    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(
                "Search for a picture of a cat, a video of a dog, and the taxonomy of each",
            )),
            name: None,
        }],
    );

    let searches = instructor_client
        .chat_completion::<Searches>(req, 3)
        .unwrap();

    searches.items.iter().for_each(|search| search.execute());
}
