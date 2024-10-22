use crate::cli::{Cli, Commands};
use crate::core::deck_loader::create_or_update_lock_file;
use crate::core::{deck_initial, deck_loader, learning, RunningCore};
use crate::repository::deck::CardItem;
use anyhow::Result;
use clap::Parser;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process;
use tokio::io::{self, AsyncBufReadExt, BufReader};

mod repository;
mod core;
mod cli;

const COMMAND_HINT_1: &str = "(q: quit | r: reveal | i: ignore)";
const COMMAND_HINT_2: &str = "(q: quit | a: easy, s: good, d: hard, f: again | i: ignore)";
const EMPTY_CARD: &str = "All cards are done!";

impl Display for CardItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_str();
        let glance = self.glance.as_ref().map(|s| format!("glance: {}\n", s)).unwrap_or(String::new());
        let content = self.content.as_str();
        let tags = match self.tags.as_ref() {
            Some(tags) => tags.join(", "),
            None => String::new()
        };
        write!(f, "{}\n{}{}\n{}", name, glance, content, tags)
    }
}

fn partial_display(card_item: &CardItem) -> String {
    let name = card_item.name.as_str();
    let glance = card_item.glance.as_ref().map(|s| format!("glance: {}\n", s)).unwrap_or(String::new());
    format!("{}\n{}", name, glance)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Init(dir_args) => {
            let dir_path: PathBuf = if dir_args.dir == "." {
                std::env::current_dir()?
            } else { Path::new(&dir_args.dir).to_path_buf() };
            deck_initial::write_initial_deck(&dir_path).await?;
            println!("Deck initialized at {:?}", dir_path);
            Ok(())
        }
        Commands::Start(dir_args) => {
            let dir_path: PathBuf = if dir_args.dir == "." {
                std::env::current_dir()?
            } else { Path::new(&dir_args.dir).to_path_buf() };
            let meta_file = deck_loader::find_deck_meta_file(&dir_path).await?;
            if meta_file.is_none() {
                eprintln!("Deck metadata file (deck.yaml) not found");
                process::exit(1);
            }
            let deck = deck_loader::read_deck_meta_file(&dir_path).await?;
            let cards = create_or_update_lock_file(&dir_path, &deck).await?;
            let mut running_core = RunningCore::new(dir_path, deck, cards).await;
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin).lines();
            'cards_loop: loop {
                let new_one = running_core.random_on_time();
                if new_one.is_none() {
                    println!("{}", EMPTY_CARD);
                    break;
                }
                let new_one = new_one.unwrap();
                let id = new_one.get_id();
                let card = running_core.cards.get(&id).unwrap();
                println!("\n{}\n{}\n", COMMAND_HINT_1, partial_display(card));
                'command_1_input_loop: loop {
                    if let Ok(Some(line)) = reader.next_line().await {
                        match line.as_str() {
                            "q" => break 'cards_loop,
                            "r" => break 'command_1_input_loop,
                            "i" => {
                                running_core.set_ignored(id).await?;
                                println!("Card ignored");
                                continue 'cards_loop;
                            },
                            _ => continue 'command_1_input_loop
                        }
                    } else {
                        break 'cards_loop;
                    }
                }
                println!("\n{}\n{}\n", COMMAND_HINT_2, card);
                'command_2_input_loop: loop {
                    if let Ok(Some(line)) = reader.next_line().await {
                        match line.as_str() {
                            "q" => break 'cards_loop,
                            "a" => {
                                running_core.next_state(id, learning::Difficulty::Easy).await?;
                                println!("Card marked as easy");
                                continue 'cards_loop;
                            },
                            "s" => {
                                running_core.next_state(id, learning::Difficulty::Good).await?;
                                println!("Card marked as good");
                                continue 'cards_loop;
                            },
                            "d" => {
                                running_core.next_state(id, learning::Difficulty::Hard).await?;
                                println!("Card marked as hard");
                                continue 'cards_loop;
                            },
                            "f" => {
                                running_core.next_state(id, learning::Difficulty::Again).await?;
                                println!("Card marked as again");
                                continue 'cards_loop;
                            },
                            "i" => {
                                running_core.set_ignored(id).await?;
                                println!("Card ignored");
                                continue 'cards_loop;
                            },
                            _ => continue 'command_2_input_loop
                        }
                    } else {
                        break 'cards_loop;
                    }
                }
            }
            Ok(())
        }
    }
}