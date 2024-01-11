use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionRequest{
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub n: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse{
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<CompletionChoices>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage{
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoices{
    pub message: Message,
    pub index: u32,
    pub finish_reason: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    pub role: String,
    pub content: String,
}