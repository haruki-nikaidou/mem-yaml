use std::collections::{HashMap, HashSet};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::repository::deck::CardItem;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardItemIdentify(pub Uuid, pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryState {
    pub last_reviewed: NaiveDateTime,
    pub interval: f32,
    pub stability: f32,
    pub difficulty: f32,
}

impl Into<fsrs::MemoryState> for &MemoryState {
    fn into(self) -> fsrs::MemoryState {
        fsrs::MemoryState {
            stability: self.stability,
            difficulty: self.difficulty,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockItem {
    pub name: Uuid,
    pub content: Uuid,
    #[serde(flatten)]
    pub state: Option<MemoryState>,
    pub ignored: bool,
}

impl LockItem {
    pub fn new_from_card(card: &CardItem) -> Self {
        let CardItemIdentify(hashed_name, hashed_content) = card.get_id();
        LockItem {
            name: hashed_name,
            content: hashed_content,
            state: None,
            ignored: false,
        }
    }
    pub fn get_id(&self) -> CardItemIdentify {
        CardItemIdentify(self.name, self.content)
    }
}

pub fn update_lock_item_list(old_list: Vec<LockItem>, new_content: &Vec<CardItem>) -> Vec<LockItem> {
    // Get the ids of the old list and the new content
    let old_ids: HashSet<_> = old_list.iter().map(|item| item.get_id()).collect();
    let new_ids: HashSet<_> = new_content.iter().map(|item| item.get_id()).collect();

    // Get the ids that are new and removed
    let new_item_ids: Vec<_> = new_ids.difference(&old_ids).collect();
    let removed_item_ids: Vec<_> = old_ids.difference(&new_ids).collect();

    // use hash map to get fast access to the items
    let new_content_map: HashMap<CardItemIdentify, &CardItem> = new_content
        .iter().map(|item| (item.get_id(), item)).collect();
    let old_list_map: HashMap<CardItemIdentify, &LockItem> = old_list
        .iter().map(|item| (item.get_id(), item)).collect();

    let new_content: Vec<_> = new_item_ids.iter().map(|id| {
        LockItem::new_from_card(new_content_map[id])
    }).collect();
    let old_list_filtered: Vec<_> = old_list_map.into_iter().filter_map(|(id, item)| {
        if removed_item_ids.contains(&&id) {
            None
        } else {
            Some(item.to_owned())
        }
    }).collect();
    let mut new_list = old_list_filtered;
    new_list.extend(new_content);
    new_list.sort_by(|a, b| a.get_id().0.cmp(&b.get_id().0));
    new_list
}


#[cfg(test)]
mod test {
    use uuid::Uuid;
    use crate::repository::deck::CardItem;
    use crate::repository::lock::{LockItem, MemoryState};

    #[test]
    fn test_update_lock_item_list() {
        // name1: 65c5dc60-15a2-5411-8cfe-8fd050fcbb9b
        let name1 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"name_1");
        // content1: a5351c64-fc3b-5239-9165-87a3dff27aef
        let content1 = Uuid::new_v5(&name1, b"content_1");
        // name2: 8cc977b5-92e0-5792-b446-39c3acace751
        let name2 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"name_2");
        // content2: 8e3da27b-cd19-5c09-b47d-4566371808e1
        let content2 = Uuid::new_v5(&name2, b"content_2");
        // name3: 132c985a-1de4-5e57-8352-f8659cfdbe00
        let name3 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"name_3");
        // content3: ff758803-89c6-566b-980c-b9e84de16acf
        let content3 = Uuid::new_v5(&name3, b"content_3");

        let old_list = vec![
            LockItem {
                name: name2,
                content: content2,
                state: None,
                ignored: false,
            },
            LockItem {
                name: name1,
                content: content1,
                state: MemoryState {
                    last_reviewed: chrono::NaiveDateTime::UNIX_EPOCH,
                    interval: 0.3,
                    stability: 0.4,
                    difficulty: 0.5,
                }.into(),
                ignored: false,
            },
        ];
        let new_content = vec![
            CardItem {
                name: "name_1".to_string(),
                glance: None,
                content: "content_1".to_string(),
                tags: None,
            },
            CardItem {
                name: "name_3".to_string(),
                glance: None,
                content: "content_3".to_string(),
                tags: None,
            },
        ];
        let new_list = super::update_lock_item_list(old_list, &new_content);

        assert_eq!(new_list.len(), 2);

        assert_eq!(new_list[0].name, name3);
        assert_eq!(new_list[0].content, content3);
        assert_eq!(new_list[1].name, name1);
        assert_eq!(new_list[1].content, content1);


        assert!(new_list[1].state.is_some());

        let state = new_list[1].state.as_ref().unwrap();
        assert_eq!(state.last_reviewed, chrono::NaiveDateTime::UNIX_EPOCH);
        assert_eq!(state.interval, 0.3);
        assert_eq!(state.stability, 0.4);
        assert_eq!(state.difficulty, 0.5);
    }
}