# instructor-rs

> We also support llama-cpp!

Instructor is a Rust library that makes it a breeze to work with structured outputs from large language models (LLMs). it provides a simple and easy API to help maanage LLM Workflows by abstrating away validation, retries and streamning responses.

Now, let's see Instructor in action with a simple example:

```rust
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

```

## Structured Validation

We can use native inbuilt serde functions in order to handle validation of specific values.

```rust
let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
let instructor_client = from_openai(client);

#[derive(InstructMacro, Debug, Serialize, Deserialize)]
// This represents a single user
struct UserInfo {
    // This represents the name of the user
    #[validate(custom = "validate_uppercase")]
    name: String,
    // This represents the age of the user
    age: u8,
}

#[validate]
fn validate_uppercase(name: &String) -> Result<String, String> {
    if name.chars().any(|c| c.is_lowercase()) {
        return Err(format!(
            "Name '{}' should be entirely in uppercase. Examples: 'TIMOTHY', 'JANE SMITH'",
            name
        ));
    }
    Ok(name.to_uppercase())
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

println!("{}", result.name); // JOHN DOE
println!("{}", result.age); // 30
```
