use std::path::PathBuf;

use lindera::{
    mode::Mode,
    tokenizer::{
        DictionaryConfig, DictionaryKind, DictionarySourceType, TokenizerConfig,
        UserDictionaryConfig,
    },
};
use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[mry::mry]
pub fn dictionary_setup(
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> TokenizerConfig {
    let mut dictionary = DictionaryConfig {
        kind: DictionaryKind::IPADIC,
        path: None,
    };

    if let Some(dict_path) = dictionary_path {
        dictionary.path = Some(PathBuf::from(dict_path))
    }

    let mut config = TokenizerConfig {
        dictionary,
        mode: Mode::Normal,
        user_dictionary: None,
    };

    if let Some(user_dict_path) = user_dictionary {
        let user_dictionary = Some(UserDictionaryConfig {
            kind: DictionaryKind::IPADIC,
            source_type: DictionarySourceType::Csv,
            path: PathBuf::from(user_dict_path),
        });

        config.user_dictionary = user_dictionary
    }

    config
}

pub async fn create_pool() -> PgPool {
    let ip_db_url = &format!(
        "postgres://{}:{}@{}:{}",
        SETTINGS.db_user, SETTINGS.db_password, SETTINGS.db_host, SETTINGS.db_port,
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(ip_db_url)
        .await
        .unwrap()
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().unwrap());
pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
}

impl Settings {
    fn new() -> anyhow::Result<Self> {
        Ok(envy::from_env::<Settings>()?)
    }
}
