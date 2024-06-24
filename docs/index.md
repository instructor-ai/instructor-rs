# What is instructor-ai?

Instructor makes it easy to get structured data like JSON from LLMs like GPT-3.5, GPT-4, GPT-4-Vision, and open-source models including Mistral/Mixtral, Anyscale, Ollama, and llama-cpp-python.

Our library is currently in active development and we're looking for active contributors interested in contributing!

## Getting Started

To install `instructor-ai`, you'll need to add the following to your cargo.toml file

```toml
instructor-ai = "0.1.0"
instruct-macros = "0.1.1"
openai-api-rs = "4.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
instruct-macros-types = "0.1.2"
```

Getting started with structured extraction is then as simple as

```rust
use std::env;
use instruct_macros::InstructMacro;
use instruct_macros_types::{ParameterInfo, StructInfo};
use instructor_ai::from_openai;
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT3_5_TURBO,
};
use serde::{Deserialize, Serialize};

fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let instructor_client = from_openai(client);

    #[derive(InstructMacro, Debug, Serialize, Deserialize)]
    // This represents a single user
    struct UserInfo {
        // This represents the name of the user
        name: String,
        // This represents the age of the user
        age: u8,
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

    println!("{}", result.name); // John Doe
    println!("{}", result.age); // 30
}

```
