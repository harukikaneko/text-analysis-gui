#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::domain::CountByNoun;
use tauri::{command, generate_context, generate_handler, Builder};

mod domain;
mod usecase;

#[command]
fn count_by_noun(text: String) -> Result<Vec<CountByNoun>, String> {
    match usecase::text_toknizer::aggregate_group_by_noun(text) {
        Ok(items) => Ok(items),
        Err(err) => Err(format!("failed to {}", err)),
    }
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![count_by_noun])
        .run(generate_context!())
        .expect("error while running tauri application");
}
