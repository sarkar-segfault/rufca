use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FlashCard {
    pub text: String,
    pub answer: String,
    pub weight: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct FlashCardSet {
    pub name: Option<String>,
    pub author: Option<String>,
    pub total: Option<u32>,
    pub flashcards: Option<Vec<FlashCard>>
}
