#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use domain::{CountsByNoun, CountsOfNounsByYear, TextWithYears};
use futures::future::try_join_all;
use itertools::Itertools;
use tauri::{command, generate_context, generate_handler, Builder};

mod config;
mod domain;
mod usecase;

#[command]
async fn counts_by_noun(
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
async fn counts_of_nouns_by_year(
    target: TextWithYears,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<Vec<CountsOfNounsByYear>, String> {
    let handles = target
        .group_by_year()
        .0
        .into_iter()
        .map(|v| {
            usecase::token::get_tokens_by_year(v.year, v.text, &dictionary_path, &user_dictionary)
        })
        .collect_vec();
    let aggregate_target = match try_join_all(handles).await {
        Ok(v) => v.into_iter().collect_vec(),
        Err(err) => return Err(format!("failed to tokens {}", err)),
    };
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
