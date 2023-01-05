#[macro_use]
extern crate serde_derive;

mod error;
mod env;
mod communication;
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
        gpt::API_URL_DEFAULT.to_string(),
        gpt::MAX_TOKENS,
        gpt::DA_VINCI_MODEL.to_string(),
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
