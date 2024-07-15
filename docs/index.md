# Instructor

Instructor makes it easy to get structured data like JSON from LLMs like GPT-3.5, GPT-4, GPT-4-Vision, and open-source models including Mistral/Mixtral, Anyscale, Ollama, and llama-cpp-python.

> Instructor's Rust Client is in active development. This means that the API and package might change moving forward. We are looking for active contributors to the repository in the meantime to help flesh out more of the features.

## Roadmap

Here is a rough roadmap of features we'd like to implement

**Struct -> JSON parsing**

- [x] Strings
- [x] Handle Booleans
- [x] Integers
- [x] Handle String Enums
- [x] Lists
- [x] Nested Structs
- [ ] Union Types (Eg. Struct1 | Struct 2 )

**Validators**

- [ ] Support different types of integers (Eg. u8, u32, u64 -> Add a validator automatically which checks that we have a value > 0 and < max value)
- [ ] Validation Context (Eg. We can validate citations by passing in original passage )

**Clients**

- [x] OpenAI
- [ ] Anthropic
- [ ] Cohere
- [ ] Gemini
- [ ] Mistral
- [ ] Llama-cpp

**CLI**

- [ ] Support Batch jobs using Instructor
- [ ] Support Fine-Tuning jobs using instructor
- [ ] Monitor Usage

## Getting Started

To install `instructor-ai`, you'll need to add the following to your cargo.toml file

```toml
instructor-ai = "0.1.8"
instruct-macros = "0.1.8"
openai-api-rs = "4.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
instruct-macros-types = "0.1.8"
```

Getting started with structured extraction is then as simple as declaring a new struct with the `InstructMacro` and importing the `ParamterInfo` and `StructInfo` types.

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
