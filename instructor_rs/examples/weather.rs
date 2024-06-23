use anyhow::{anyhow, Result};
use async_trait::async_trait;
use instruct_model::{ChatCompletionResponse, InstructModel, Instructor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, InstructModel)]
struct WeatherForecast {
    date: String,
    temperature_c: i32,
    temperature_f: i32,
    details: WeatherDetails,
}

#[derive(Debug, Serialize, Deserialize, InstructModel)]
struct WeatherDetails {
    avg_humidity: i32,
    avg_wind_speed: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Instructor::from_openai(std::env::var("OPENAI_API_KEY")?);
    let prompt = &format!(
        "Extract weather forecast from: On 2022-01-01, the temperature is 20C and 68F, with an average humidity of 50% and an average wind speed of 10 mph. Example object {}",
        serde_json::to_string(&WeatherForecast {
            date: "2022-01-01".to_string(),
            temperature_c: 20,
            temperature_f: 68,
            details: WeatherDetails {
                avg_humidity: 50,
                avg_wind_speed: 10,
            },
        })?
    );
    let weather_forecast: WeatherForecast = client.extract(prompt).await?;
    println!("Date: {}", weather_forecast.date);
    println!("Temperature (C): {}", weather_forecast.temperature_c);
    println!("Temperature (F): {}", weather_forecast.temperature_f);

    Ok(())
}
