use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::repository::lock::CardItemIdentify;

#[derive(Debug, Clone, Deserialize)]
pub struct CardItem {
    pub name: String,
    pub glance: Option<String>,
    pub content: String,
    pub tags: Option<Vec<String>>
}

impl CardItem {
    pub fn get_id(&self) -> CardItemIdentify {
        let hashed_name = Uuid::new_v5(&Uuid::NAMESPACE_OID, self.name.as_bytes());
        let hashed_content = Uuid::new_v5(&hashed_name, self.content.as_bytes());
        CardItemIdentify(hashed_name, hashed_content)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    pub description: Option<String>,
    pub card_files: Vec<String>,
    #[serde(default = "default_algorithm")]
    pub algorithm: RepeatAlgorithm,
    pub fsrs_option: Option<FsrsOption>,
}

fn default_algorithm() -> RepeatAlgorithm {
    RepeatAlgorithm::Fsrs
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RepeatAlgorithm {
    #[default]
    Fsrs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FsrsOption {
    #[serde(default = "default_retention")]
    pub retention: f32,
}

fn default_retention() -> f32 {
    0.75
}