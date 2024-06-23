use anyhow::{anyhow, Result};
use async_trait::async_trait;
use instruct_model::{ChatCompletionResponse, InstructModel, Instructor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, InstructModel)]
struct UserInfo {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Instructor::from_openai(std::env::var("OPENAI_API_KEY")?);

    let prompt = "Extract user info from: John Doe is 30 years old";
    let user_info: UserInfo = client.extract(prompt).await?;
    println!("Name: {}", user_info.name);
    println!("Age: {}", user_info.age);

    Ok(())
}
