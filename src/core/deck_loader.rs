use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use tokio::fs;
use crate::repository::deck::{CardItem, Deck};
use crate::repository::lock::{update_lock_item_list, CardItemIdentify, LockItem};

const DECK_METADATA_1: &str = "deck.yaml";
const DECK_METADATA_2: &str = "deck.yml";
const DECK_METADATA_3: &str = "deck.json";

pub async fn find_deck_meta_file(dir: &Path) -> Result<Option<PathBuf>> {
    let mut dir = fs::read_dir(dir).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_ascii_lowercase();
                if file_name == DECK_METADATA_1 || file_name == DECK_METADATA_2 || file_name == DECK_METADATA_3 {
                    return Ok(Some(path));
                }
            }
        }
    }
    Ok(None)
}
pub async fn read_deck_meta_file(dir: &Path) -> Result<Deck> {
    let meta_file = find_deck_meta_file(dir).await?;
    if let Some(meta_file) = meta_file {
        let content = fs::read_to_string(meta_file).await?;
        let deck: Deck = serde_yaml::from_str(&content)?;
        Ok(deck)
    } else {
        Err(anyhow::anyhow!("Deck metadata file not found"))
    }
}

const LOCK_FILE: &str = "deck.lock";

pub async fn read_lock_file(dir: &Path) -> Result<Vec<LockItem>> {
    let lock_file = dir.join(LOCK_FILE);
    if !lock_file.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(lock_file).await?;
    let lock_list: Vec<LockItem> = serde_json::from_str(&content)?;
    Ok(lock_list)
}

pub async fn write_lock_file(dir: &Path, lock_list: &Vec<LockItem>) -> Result<()> {
    let lock_file = dir.join(LOCK_FILE);
    let content = serde_json::to_string_pretty(lock_list)?;
    fs::write(lock_file, content).await?;
    Ok(())
}

async fn read_cards(file: &Path) -> Result<Vec<CardItem>> {
    let content = fs::read_to_string(file).await?;
    let cards: Vec<CardItem> = serde_yaml::from_str(&content)?;
    Ok(cards)
}

pub async fn create_or_update_lock_file(dir: &Path, deck: &Deck) -> Result<HashMap<CardItemIdentify, CardItem>> {
    let existing_lock = read_lock_file(dir).await.unwrap_or_else(|_| Vec::new());
    let cards_files_name = &deck.card_files;
    let mut cards = Vec::new();
    for file_name in cards_files_name {
        let file = dir.join(file_name);
        let file_cards = read_cards(&file).await?;
        cards.extend(file_cards);
    }
    let new_lock = update_lock_item_list(existing_lock, &cards);
    write_lock_file(dir, &new_lock).await?;
    let cards: HashMap<CardItemIdentify, CardItem> = cards.into_iter().map(|card| (card.get_id(), card)).collect();
    Ok(cards)
}