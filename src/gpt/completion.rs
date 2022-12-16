use serde::Deserialize;

#[derive(Deserialize)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u32>,
    pub finish_reason: String
}

#[derive(Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32
}