use anyhow::Result;

pub const SIMPLE_DECK: &str = r#"name: Hello Mem YAML
description: A minimal example of a Deck metadata file
card_files:
  - cards_1.yml
"#;

pub const SIMPLE_CARDS: &str = r#"- name: こんにちわ
  content: Hello
  tags:
    - greeting
    - sentence
- name: せかい
  glance: noun
  content: World
"#;

pub(crate) async fn write_initial_deck(dir: &std::path::Path) -> Result<()> {
    let deck_file = dir.join("deck.yaml");
    let cards_file = dir.join("cards_1.yml");
    tokio::fs::write(deck_file, SIMPLE_DECK).await?;
    tokio::fs::write(cards_file, SIMPLE_CARDS).await?;
    Ok(())
}