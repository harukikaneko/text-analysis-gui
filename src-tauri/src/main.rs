#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{generate_context, generate_handler, Builder};

mod config;
mod domain;
mod driver;
mod gateway;
mod rest;
mod usecase;

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![
            rest::noun::counts_by_noun,
            rest::noun::counts_of_nouns_by_year,
            rest::noun::create_of_nouns_by_year
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
