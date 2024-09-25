use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    parts: Vec<Part>,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut conversations: HashMap<String, Vec<Message>> = HashMap::new();

    println!("GPT CLI");

    loop {
        let mut input = String::new();

        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Failed to read line");
        println!();

        if input.trim() == "exit" {
            break;
        } else if input.trim() == "switch" || input.trim() == "clear" || input.trim() == "new" {
            // TODO: split all of them and give each one there functionality
            conversations = HashMap::new();
            continue;
        } else if input.trim() == "help" {
            println!("Commands:");
            println!("new    - Start a new conversation");
            println!("switch - Switch to another conversation");
            println!("clear  - Clear the conversation history");
            println!("exit   - Exit the program\n");
            continue;
        }

        let message = Message {
            role: "user".to_string(),
            parts: vec![Part {
                text: input.trim().to_string(),
            }],
        };

        let entry = conversations
            .entry("conversation_1".to_string())
            .or_insert(vec![]);

        entry.push(message);

        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
            api_key
        );

        let contents: Vec<_> = conversations["conversation_1"]
            .iter()
            .map(|message| {
                json!({
                    "role": message.role,
                    "parts": message.parts.iter().map(|part| json!({"text": part.text})).collect::<Vec<_>>()
                })
            })
            .collect();

        let json_body = json!({
            "contents": contents
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
