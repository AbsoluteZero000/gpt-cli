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

fn switch(conversations: &HashMap<String, Vec<Message>>) -> String {
    println!("Switching to another conversation...");
    let mut new_conversation_id = String::new();
    loop {
        println!("Choose conversation:");
        for c in conversations.keys() {
            println!("{}", c);
        }

        print!("\nConversation name: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut new_conversation_id)
            .expect("Failed to read line");

        new_conversation_id = new_conversation_id.trim().to_string();

        if !conversations.contains_key(&new_conversation_id) {
            println!("Conversation not found. Please enter a valid conversation name:");
        } else {
            break;
        }
    }
    new_conversation_id
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut conversations: HashMap<String, Vec<Message>> = HashMap::new();
    let _ = stdout().flush();
    let mut conversation_id = String::new();

    println!("GPT CLI\n");
    print!("Enter the conversation name: ");
    stdin()
        .read_line(&mut conversation_id)
        .expect("Failed to read line");

    conversation_id = conversation_id.trim().to_string();
    conversations.insert(conversation_id.clone(), vec![]);

    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "exit" => break,
            "switch" => conversation_id = switch(&conversations),
            "clear" => {
                conversations = HashMap::new();
                conversation_id = String::new();
                println!("Conversation cleared. Please enter a new conversation name:");
                print!("> ");

                let _ = stdout().flush();
                stdin()
                    .read_line(&mut conversation_id)
                    .expect("Failed to read line");
                conversation_id = conversation_id.trim().to_string();
                conversations.insert(conversation_id.clone(), vec![]);
            }
            "new" => {
                conversation_id = String::new();
                print!("Enter the conversation name: ");
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut conversation_id)
                    .expect("Failed to read line");
                conversation_id = conversation_id.trim().to_string();
                conversations.insert(conversation_id.clone(), vec![]);
            }
            "help" => {
                println!("Commands:");
                println!("new    - Start a new conversation");
                println!("switch - Switch to another conversation");
                println!("clear  - Clear the conversation history");
                println!("exit   - Exit the program");
            }
            _ => {
                let message = Message {
                    role: "user".to_string(),
                    parts: vec![Part {
                        text: input.to_string(),
                    }],
                };
                conversations
                    .entry(conversation_id.clone())
                    .or_default()
                    .push(message);

                let api_key = env::var("API_KEY").expect("API_KEY must be set");
                let url = format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
                    api_key
                );

                let contents: Vec<_> = conversations[&conversation_id]
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
                    .unwrap_or("No response");

                println!("GEMINI:\n{}", resp);

                let ai_message = Message {
                    role: "model".to_string(),
                    parts: vec![Part {
                        text: resp.to_string(),
                    }],
                };
                conversations
                    .entry(conversation_id.clone())
                    .or_default()
                    .push(ai_message);
            }
        }
    }
    Ok(())
}
