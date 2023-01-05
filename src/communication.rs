#[derive(Debug, Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    //pub response: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String, // I do not care about the rest.
}