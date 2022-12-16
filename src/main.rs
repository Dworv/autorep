use autorep::GptClient;

fn main() {
    let client = GptClient::new("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".into());
    let res = client.get_completions("tell me a story about among us".into(), 2);
    println!("{res:?}");
}
