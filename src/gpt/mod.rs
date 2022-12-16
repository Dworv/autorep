use reqwest::blocking::Client as RequestClient;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{to_string, from_slice};

mod completion;
mod request;

use completion::Completion;
use request::CreateCompletionRequest;

pub struct GptClient {
    key: String,
    client: RequestClient
}

impl GptClient {
    pub fn new(key: String) -> Self {
        Self {
            key: format!("Bearer {}", key),
            client: RequestClient::new()
        }
    }

    pub fn get_completions(&self, prompt: String, num: u32) -> Vec<String> {
        let content = CreateCompletionRequest {
            prompt: Some(prompt),
            n: Some(num),
            max_tokens: Some(50),
            stop: Some(vec!["\n".into()]),
            model: "text-davinci-003".into(),
            ..Default::default()
        };
        let res = self.client.get("https://api.openai.com/v1/completions")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, self.key.clone())
            .body(to_string(&content).unwrap())
            .send()
            .unwrap()
            .text()
            .unwrap();
        let completion: Completion = from_slice(res.as_bytes()).unwrap();
        completion.choices.into_iter().map(|x| x.text).collect()
    }
}