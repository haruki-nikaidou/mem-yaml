use std::collections::HashMap;
use fsrs::FSRS;
use rand::prelude::SliceRandom;
use crate::core::deck_loader::write_lock_file;
use anyhow::Result;
use crate::repository::deck::{CardItem, Deck};
use crate::repository::lock::{CardItemIdentify, LockItem};

pub mod deck_loader;
pub mod learning;
pub mod deck_initial;

pub struct RunningCore {
    pub working_dir: std::path::PathBuf,
    deck: Deck,
    fsrs: FSRS,
    pub(crate) cards: HashMap<CardItemIdentify, CardItem>,
    lock_file: HashMap<CardItemIdentify, LockItem>
}

impl RunningCore {
    pub async fn new(working_dir: std::path::PathBuf, deck: Deck, cards: HashMap<CardItemIdentify, CardItem>) -> Self {
        let lock_file = deck_loader::read_lock_file(&working_dir).await.unwrap();
        let lock_file = lock_file.into_iter().map(|item| (item.get_id(), item)).collect();
        let fsrs = FSRS::new(Some(&[])).unwrap();
        RunningCore {
            working_dir,
            lock_file,
            fsrs,
            cards,
            deck
        }
    }
    pub fn random_on_time(&self) -> Option<LockItem> {
        let on_time: Vec<_> = self
            .lock_file
            .iter()
            .filter(|(_,s)| s.filter_on_time())
            .collect();
        if on_time.is_empty() {
            None
        } else {
            Some(on_time.choose(&mut rand::thread_rng()).unwrap().1.to_owned())
        }
    }
    pub async fn next_state(&mut self, id: CardItemIdentify, difficulty: learning::Difficulty) -> Result<()> {
        let retention = match self.deck.fsrs_option {
            None => crate::repository::deck::default_retention(),
            Some(ref fsrs_option) => fsrs_option.retention
        };
        let lock_item = self.lock_file.get_mut(&id).unwrap();
        lock_item.next_state(&self.fsrs, retention, difficulty);
        write_lock_file(&self.working_dir, &self.lock_file.values().cloned().collect()).await?;
        Ok(())
    }
    pub async fn set_ignored(&mut self, id: CardItemIdentify) -> Result<()> {
        let lock_item = self.lock_file.get_mut(&id).unwrap();
        lock_item.set_ignored(true);
        write_lock_file(&self.working_dir, &self.lock_file.values().cloned().collect()).await?;
        Ok(())
    }
}