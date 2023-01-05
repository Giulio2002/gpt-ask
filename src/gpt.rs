extern crate reqwest;
extern crate serde;

use crate::{
    error::Error,
    communication::{
        CompletionResponse,
        CompletionRequest,
    },
};

// Constants for default API URL and a specific model name
pub const API_URL_DEFAULT: &str = "https://api.openai.com/v1";
pub const DA_VINCI_MODEL: &str = "text-davinci-003";

// Maximum number of tokens that the API should return in its response
pub const MAX_TOKENS: usize = 4000;

// Struct for holding client configuration and state
pub struct OpenAIClient {
    api_key: String,
    api_url: String,
    max_tokens: usize,
    model: String,
}

impl OpenAIClient {
    // Constructor for creating a new OpenAIClient
    pub fn new(api_key: String, api_url: String, max_tokens: usize, model: String) -> OpenAIClient {
        OpenAIClient {api_key, api_url, max_tokens, model}
    }

    // Asynchronously send a completion request to the API and return the response
    pub async fn ask(self: &Self, prompt: String) -> Result<String, Error> {
        // Create a new HTTP client
        let client = reqwest::Client::new();

        // Construct the completion request
        let request: CompletionRequest = CompletionRequest {
            model: self.model.to_owned(),
            max_tokens: self.max_tokens,
            prompt,
        };

        // Construct the endpoint URL
        let endpoint = format!("{}/completions", self.api_url);

        // Send the request and store the response
        let res = client
            .post(endpoint)
            .json(&request)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        // Get the response body as a string
        let text = res.text().await?;

        // Deserialize the response JSON
        let response: CompletionResponse = serde_json::from_str(&text)?;

        // Return the text of the first choice in the response
        Ok(response.choices[0].text.to_owned())
    }
}
