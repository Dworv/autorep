use std::collections::HashMap;

use serde::Serialize;

/// A request to create a completion
#[derive(Serialize, Default, Debug)]
pub struct CreateCompletionRequest {
    pub model: String,
    pub prompt: Option<String>,
    pub suffix: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<u32>,
    pub top_p: Option<u32>,
    pub n: Option<u32>,
    pub stream: Option<bool>,
    pub logprobs: Option<u32>,
    pub echo: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub presence_penalty: Option<u32>,
    pub frequency_penalty: Option<u32>,
    pub best_of: Option<u32>,
    pub logit_bias: Option<HashMap<String, i8>>,
    pub user: Option<String>
}