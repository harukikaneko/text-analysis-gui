use tauri::command;

use crate::domain::{CountsByNoun, CountsOfNounsByYear};

use crate::usecase;

#[command]
pub async fn counts_by_noun(
    text: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<Vec<CountsByNoun>, String> {
    let tokens = match usecase::token::get_tokens(text, dictionary_path, user_dictionary) {
        Ok(tokens) => tokens,
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("failed to tokens {:?}", err))
            }
        }
    };
    match usecase::noun::aggregate_group_by_noun(tokens) {
        Ok(items) => Ok(items),
        Err(err) => {
            tracing::error!("{:?}", err);
            Err(format!("failed to {:?}", err))
        }
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
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("Failed csv {:?}", err))
            }
        }
    };

    let aggregate_target = match usecase::token::get_tokens_by_year_handles_join(
        csv_list,
        dictionary_path,
        user_dictionary,
    )
    .await
    {
        Ok(v) => v,
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("failed to tokens {:?}", err))
            }
        }
    };

    match usecase::noun::aggregate_counts_of_nouns_by_year(aggregate_target) {
        Ok(items) => Ok(items),
        Err(err) => {
            tracing::error!("{:?}", err);
            Err(format!("failed to {:?}", err))
        }
    }
}

#[command]
pub async fn create_of_nouns_by_year(
    csv_path: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> Result<(), String> {
    let csv_list = match usecase::csv::read_csv(csv_path).await {
        Ok(csv) => csv,
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("Failed csv {:?}", err))
            }
        }
    };

    let translated_csv = match usecase::translate::texts_translate(csv_list).await {
        Ok(translated) => translated,
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("Failed translate {:?}", err))
            }
        }
    };

    let create_target = match usecase::token::get_tokens_by_year_handles_join(
        translated_csv,
        dictionary_path,
        user_dictionary,
    )
    .await
    {
        Ok(v) => v,
        Err(err) => {
            return {
                tracing::error!("{:?}", err);
                Err(format!("failed to tokens {:?}", err))
            }
        }
    };

    match usecase::noun::create_of_nouns_by_year(create_target).await {
        Ok(_) => Ok(()),
        Err(err) => {
            tracing::error!("{:?}", err);
            Err(format!("failed to {:?}", err))
        }
    }
}
