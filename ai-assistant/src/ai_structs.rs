use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CompletionRequest{
    prompt: String,
    temperature: f32,
    max_tokens: u32,
    n: u32,
    stop: Option<String>,
    engine: String
}

#[derive(Serialize, Deserialize, Debug)]
struct LogProbs {
    token_logprobs: Vec<f32>,
    text_offset: Vec<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionChoices{
    text: String,
    index: u32,
    logprobs: Option<LogProbs>
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionResponse{
    choices: Vec<CompletionChoices>
}
