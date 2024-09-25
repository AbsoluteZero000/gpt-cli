use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    loop {
        let mut input = String::new();

        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Failed to read line");
        println!();

        if input.trim() == "exit" {
            break;
        }

        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
            api_key
        );

        let json_body = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": input.trim()
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

        let data = resp.json::<serde_json::Value>()?;
        let resp = data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("");

        println!("GEMINI:\n {}", resp);
    }

    Ok(())
}
