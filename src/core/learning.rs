use crate::repository::lock::{LockItem, MemoryState};
use chrono::{Duration, Local, TimeZone, Utc};
use fsrs::FSRS;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Good,
    Hard,
    Again,
}

impl LockItem {
    pub fn set_ignored(&mut self, ignored: bool) {
        self.ignored = ignored;
    }
    pub fn next_state(&mut self, fsrs: &FSRS, retention: f32, difficulty: Difficulty) {
        let last_reviewed = Utc::now().naive_utc();
        let new_state = match self.state {
            None => fsrs.next_states(None, retention, 0),
            Some(ref state) => {
                let last_review_date = Local.from_local_datetime(&last_reviewed).unwrap();
                let just_now = Local::now();
                let interval = (just_now - last_review_date).num_days();
                fsrs.next_states(Some(state.into()), retention, interval as u32)
            }
        }.unwrap();
        let new_state = match difficulty {
            Difficulty::Easy => new_state.easy,
            Difficulty::Good => new_state.good,
            Difficulty::Hard => new_state.hard,
            Difficulty::Again => new_state.again,
        };
        let new_memory_state = MemoryState {
            last_reviewed,
            interval: new_state.interval,
            stability: new_state.memory.stability,
            difficulty: new_state.memory.difficulty,
        };
        self.state = Some(new_memory_state);
    }
    pub fn filter_on_time(&self) -> bool {
        let now = Utc::now().naive_utc();
        if self.state.is_none() {
            return true;
        }
        let state = self.state.as_ref().unwrap();
        let last_reviewed = state.last_reviewed;
        let interval = Duration::seconds((state.interval * 24.0 * 60.0 * 60.0) as i64);
        let next_review = last_reviewed + interval;
        now >= next_review
    }
}