use std::fs;

use tauri::command;

use crate::domain::{CountsByNoun, CountsOfNounsByYear, TextWithYears, TextWithYear};
use futures::future::try_join_all;
use itertools::Itertools;

use crate::usecase;

#[command]
pub async fn counts_by_noun(
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
pub async fn counts_of_nouns_by_year(
    csv_path: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<Vec<CountsOfNounsByYear>, String> {
    let csv_list = TextWithYears(read_csv(csv_path).unwrap());

    let handles = csv_list
        .group_by_year()
        .0
        .into_iter()
        .map(|v| {
            usecase::token::get_tokens_by_year(v.year, v.r#abstract, &dictionary_path, &user_dictionary)
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

fn read_csv(file_path: String) -> anyhow::Result<Vec<TextWithYear>> {
    let mut csv_list = Vec::new();

    let csv_text = fs::read_to_string(file_path)?;
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        let record = result?.deserialize(None)?;
        csv_list.push(record);
    }

    Ok(csv_list)
}