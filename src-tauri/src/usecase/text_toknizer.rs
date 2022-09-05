use std::path::PathBuf;

use crate::domain::{CountByNoun, Tokens};
use lindera::{
    mode::Mode,
    tokenizer::{
        DictionaryConfig, DictionaryKind, DictionarySourceType, Tokenizer, TokenizerConfig,
        UserDictionaryConfig,
    },
    LinderaResult,
};

pub fn aggregate_group_by_noun(
    word: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> LinderaResult<Vec<CountByNoun>> {
    let config = setup(dictionary_path, user_dictionary);

    let tokenizer = Tokenizer::with_config(config)?;
    let tokens = Tokens(tokenizer.tokenize(&word)?);
    let exclude_non_nouns = tokens.exclude_non_nouns();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

#[mry::mry]
fn setup(
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

#[cfg(test)]
mod test {
    use crate::domain::Noun;

    use super::*;

    #[test]
    #[mry::lock(setup)]
    fn test_aggregate_group_by_noun() {
        let word = "東京は".into();
        let expected = vec![CountByNoun {
            noun: Noun("東京".into()),
            counts: 1,
        }];

        let dictionary = DictionaryConfig {
            kind: DictionaryKind::IPADIC,
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            mode: Mode::Normal,
            user_dictionary: None,
        };

        mock_setup(None, None).returns(config);

        let actual = aggregate_group_by_noun(word, None, None).unwrap();
        assert_eq!(actual, expected)
    }
}
