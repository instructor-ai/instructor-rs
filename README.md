# instructor-rs

Instructor is a RUst library that makes it a breeze to work with structured outputs from large language models (LLMs). it provides a simple and easy API to help maanage LLM Workflows by abstrating away validation, retries and streamning responses.

Now, let's see Instructor in action with a simple example:

```rust
#[derive(InstructModel)]
struct UserInfo{
    name: str
    age: u8
}

let client = Instructor::from_openai(Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()));

let user = ChatCompletionRequest::new(
        GPT3_5_TURBO_0613.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("John Doe is 30 years old")),
            name: None,
        }],
        vec![UserInfo]
)

println!("{}", UserInfo.name) // John Doe
println!("{}", UserInfo.age)  // 30
```
