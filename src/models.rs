use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlashCard {
    pub question: String,
    pub answer: String,
    pub weight: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FlashCardSet {
    pub name: Option<String>,
    pub author: Option<String>,
    pub total: u32,
    pub flashcards: Vec<FlashCard>,
}
