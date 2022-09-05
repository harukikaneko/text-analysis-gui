#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use domain::{CountsByNoun, CountsOfNounsByYear, TextWithYears};
use tauri::{command, generate_context, generate_handler, Builder};

mod config;
mod domain;
mod usecase;

#[command]
fn counts_by_noun(
    text: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<Vec<CountsByNoun>, String> {
    let tokens = match usecase::token::get_tokens(text, dictionary_path, user_dictionary) {
        Ok(tokens) => tokens,
        Err(err) => return Err(format!("failed to tokens {}", err)),
    };
    match usecase::noun::aggregate_group_by_noun(tokens) {
        Ok(items) => Ok(items),
        Err(err) => Err(format!("failed to {}", err)),
    }
}

#[command]
fn counts_of_nouns_by_year(
    aggregate_target: TextWithYears,
) -> Result<Vec<CountsOfNounsByYear>, String> {
    match usecase::noun::aggregate_counts_of_nouns_by_year(aggregate_target) {
        Ok(items) => Ok(items),
        Err(err) => Err(format!("failed to {}", err)),
    }
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![counts_by_noun, counts_of_nouns_by_year])
        .run(generate_context!())
        .expect("error while running tauri application");
}
