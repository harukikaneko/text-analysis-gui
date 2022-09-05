use std::path::PathBuf;

use lindera::{
    mode::Mode,
    tokenizer::{
        DictionaryConfig, DictionaryKind, DictionarySourceType, TokenizerConfig,
        UserDictionaryConfig,
    },
};

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
