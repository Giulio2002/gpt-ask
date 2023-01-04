use std::env;

pub fn get_openai_key() -> Result<String, std::env::VarError> {
    env::var("OPENAI_KEY")
}