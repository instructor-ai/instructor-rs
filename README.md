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


## How to run examples

Try out the `readme`, `simple` and `weather` examples by running the following commands:

```bash
OPENAI_API_KEY=sk-your-key \
cargo run -p instructor-rs \
--example simple
```

## Note

This implementation is 100% in development was just hacked together for reference and to explore the api shown on the readme. It is not intended to be used in production, and likely misses a lot of the projects core features.