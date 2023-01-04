#[macro_use]
extern crate serde_derive;

mod error;
mod env;
mod gpt;

use std::process::exit;

#[tokio::main]
async fn main() {
    // Retreive key
    let api_key = match env::get_openai_key() {
        Ok(k) => k,
        Err(err) => {
            println!("$OPENAI_KEY is not set: {}", err.to_string());
            exit(0);
        }
    };
    let chat_gpt = gpt::OpenAIClient::new(
        api_key, 
        gpt::ApiUrlDefault.to_string(),
        5000,
        0.5,
        1
    );
    let answer = match chat_gpt.ask("when is christmas?".to_string()).await {
        Ok(out) => out,
        Err(err) => {
            println!("could not query openai: {}", err.to_string());
            exit(0);
        }
    };

    println!("{}", answer);
}
