

extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use crate::error::Error;

pub const ApiUrlDefault: &str = "https://api.openai.com/v1";

#[derive(Serialize, Deserialize)]
struct CompletionResponse {
    text: String,
}

pub struct OpenAIClient {
    api_key: String,
    api_url: String,
    max_tokens: usize,
    temperature: f64,
    top_p: usize,
}

impl OpenAIClient {
    pub fn new(api_key: String, api_url: String, max_tokens: usize, temperature: f64, top_p: usize) -> OpenAIClient {
        OpenAIClient {api_key, api_url, max_tokens, temperature, top_p}
    }

    pub async fn ask(self: &Self, prompt: String) -> Result<String, Error> {    
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("prompt", prompt);
        params.insert("max_tokens", self.max_tokens.to_string());
        params.insert("temperature", self.temperature.to_string());
        params.insert("top_p", self.top_p.to_string());

        let endpoint = format!("{}/completions", self.api_url);
        println!("{}", endpoint);
        let res = client
            .post(endpoint)
            .bearer_auth(&self.api_key)
            .form(&params)
            .send()
            .await?;
        let response: CompletionResponse = serde_json::from_str(
            res.
            text()
            .await?
            .as_str()
        )?;
        Ok(response.text)
    }

}

/* 
fn call_refactor(code_input: str, output_file: str) {
    let client = openai::Client::new("YOUR_API_KEY");

    let mut request = CompletionRequest::new("YOUR_PROMPT_HERE");
    request.max_tokens = Some(1024);

    let response: CompletionResponse = client.complete(request).expect("Failed to complete prompt");
    println!("{}", response.text);
}*/
