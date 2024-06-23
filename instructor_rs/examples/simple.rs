use anyhow::{anyhow, Result};
use async_trait::async_trait;
use instruct_model::{ChatCompletionResponse, InstructModel, Instructor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, InstructModel)]
struct SuperheroInfo {
    alias: String,
    real_name: String,
    superpower: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Instructor::from_openai(std::env::var("OPENAI_API_KEY")?);

    let prompt = &format!(
        "Extract superhero info from: The superhero known as 'Shadow Phantom' is actually Jane Doe, who possesses the power of invisibility and is 30 years old. The object should be {}",
        serde_json::to_string(&SuperheroInfo {
            alias: "Shadow Phantom".to_string(),
            real_name: "Jane Doe".to_string(),
            superpower: "invisibility".to_string(),
            age: 28,
        })?
    );
    let superhero_info: SuperheroInfo = client.extract(prompt).await?;
    println!("Alias: {}", superhero_info.alias);
    println!("Real Name: {}", superhero_info.real_name);
    println!("Superpower: {}", superhero_info.superpower);
    println!("Age: {}", superhero_info.age);

    Ok(())
}
