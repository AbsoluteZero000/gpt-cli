use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Set up the API URL and the API key from environment variable
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    // Prepare the JSON body for the request
    let json_body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": "Explain how AI works"
                    }
                ]
            }
        ]
    });

    let client = Client::new();
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()?;

    println!("Response: {:?}", resp.text()?);

    Ok(())
}
