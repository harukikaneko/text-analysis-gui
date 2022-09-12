use tauri::command;

use crate::{
    config::{create_pool, DB_POOL},
    domain::{CountsByNoun, CountsOfNounsByYear},
};
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
    let csv_list = match usecase::csv::read_csv(csv_path).await {
        Ok(csv) => csv,
        Err(err) => return Err(format!("Failed csv {}", err)),
    };

    let handles = csv_list
        .group_by_year()
        .0
        .into_iter()
        .map(|v| {
            usecase::token::get_tokens_by_year(
                v.year,
                v.r#abstract,
                &dictionary_path,
                &user_dictionary,
            )
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

#[command]
pub async fn create_of_nouns_by_year(
    csv_path: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<(), String> {
    let ip_db_pool = create_pool().await;
    DB_POOL.set(ip_db_pool).unwrap();

    let csv_list = match usecase::csv::read_csv(csv_path).await {
        Ok(csv) => csv,
        Err(err) => return Err(format!("Failed csv {}", err)),
    };

    let handles = csv_list
        .group_by_year()
        .0
        .into_iter()
        .map(|v| {
            usecase::token::get_tokens_by_year(
                v.year,
                v.r#abstract,
                &dictionary_path,
                &user_dictionary,
            )
        })
        .collect_vec();

    let create_target = match try_join_all(handles).await {
        Ok(v) => v.into_iter().collect_vec(),
        Err(err) => return Err(format!("failed to tokens {}", err)),
    };

    match usecase::noun::create_of_nouns_by_year(create_target).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("failed to {}", err)),
    }
}
