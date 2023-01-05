

extern crate reqwest;
extern crate serde;

use crate::{
    error::Error,
    communication::{
        CompletionResponse,
        CompletionRequest,
    },
};

pub const API_URL_DEFAULT: &str = "https://api.openai.com/v1";
pub const DA_VINCI_MODEL: &str = "text-davinci-003";
pub const MAX_TOKENS: usize = 4000;
pub struct OpenAIClient {
    api_key: String,
    api_url: String,
    max_tokens: usize,
    model: String,
}

impl OpenAIClient {
    pub fn new(api_key: String, api_url: String, max_tokens: usize, model: String) -> OpenAIClient {
        OpenAIClient {api_key, api_url, max_tokens, model}
    }

    pub async fn ask(self: &Self, prompt: String) -> Result<String, Error> {    
        let client = reqwest::Client::new();
        let request: CompletionRequest = CompletionRequest {
            model: self.model.to_owned(),
            max_tokens: self.max_tokens,
            prompt,
        };
        let endpoint = format!("{}/completions", self.api_url);
        let res = client
            .post(endpoint)
            .json(&request)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let text = res.text().await?;
        let response: CompletionResponse = serde_json::from_str(&text)?;
        Ok(response.choices[0].text.to_owned())
    }
}

