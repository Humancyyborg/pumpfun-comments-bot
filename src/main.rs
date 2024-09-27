use reqwest::{Client, Proxy};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use tokio::runtime::Runtime;
use rand::seq::SliceRandom; // Import for random selection

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mode = prompt_user("Choose mode: [1] Random Wallets, [2] User Wallets, [3] Like Replies, [4] Shill Wallets, [5] Thread Mode, [6] Exit: ").await;
    match mode.as_str() {
        "1" => println!("Random Wallets mode selected."),
        "2" => println!("User Wallets mode selected."),
        "3" => println!("Like Replies mode selected."),
        "4" => println!("Shill Wallets mode selected."),
        "5" => {
            let thread_id = prompt_user("Enter thread ID: ").await;
            let token = sign_in_random().await;
            let proxy = Some("http://your.proxy:port"); // Set your proxy if needed
            comment_thread(thread_id, token, proxy).await?;
        }
        "6" => {
            println!("Exiting...");
            return Ok(());
        }
        _ => println!("Invalid choice, try again."),
    }
    Ok(())
}

async fn prompt_user(query: &str) -> String {
    print!("{}", query);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

async fn sign_in_random() -> String {
    // Implement your sign-in logic here
    String::from("example_token") // Placeholder for token
}

async fn comment_thread(thread_id: String, token: String, proxy: Option<&str>) -> Result<(), Box<dyn Error>> {
    let comments_url = format!("https://pumpfun.com/thread/{}/comments", thread_id);
    
    // Create a Client builder
    let client_builder = Client::builder();

    // If a proxy is provided, set it up
    let client = if let Some(proxy_url) = proxy {
        let proxy = Proxy::http(proxy_url)?;
        client_builder.proxy(proxy).build()?
    } else {
        client_builder.build()?
    };

    // Make the GET request
    let response = client.get(&comments_url).send().await?;
    let comments: Vec<Value> = response.json().await?;
    
    // Randomly select a comment ID
    let random_comment = comments.choose(&mut rand::thread_rng()).ok_or("No comments found")?;
    let comment_id = random_comment.get("id").ok_or("Comment ID not found")?.as_str().ok_or("Comment ID is not a string")?;
    
    // Read comments from file
    let comments_file = fs::read_to_string("./comments.txt")?;
    let comments_list: Vec<&str> = comments_file.lines().collect();
    let random_comment_text = comments_list.choose(&mut rand::thread_rng()).unwrap_or(&"Default comment").to_string();
    
    // Prepare comment payload
    let post_url = format!("https://pumpfun.com/thread/{}/comment", thread_id);
    let comment_payload = serde_json::json!({
        "text": format!("#{} {}", comment_id, random_comment_text),
        "mint": thread_id,
        "token": token,
    });

    // Make the POST request
    let post_response = client.post(&post_url)
        .json(&comment_payload)
        .send()
        .await?;

    if post_response.status().is_success() {
        println!("Commented: {}", random_comment_text);
    } else {
        println!("Failed to comment: {}", post_response.status());
    }

    Ok(())
}
