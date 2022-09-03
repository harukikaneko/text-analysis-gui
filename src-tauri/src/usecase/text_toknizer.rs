use std::path::PathBuf;

use crate::domain::{CountByNoun, Tokens};
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, DictionaryKind, Tokenizer, TokenizerConfig},
    LinderaResult,
};

pub fn aggregate_group_by_noun(word: String) -> LinderaResult<Vec<CountByNoun>> {
    let config = setup_ipadic_neologd();

    let tokenizer = Tokenizer::with_config(config)?;
    let tokens = Tokens(tokenizer.tokenize(&word)?);
    let exclude_non_nouns = tokens.exclude_non_nouns();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

#[mry::mry]
fn setup_ipadic_neologd() -> TokenizerConfig {
    let dictionary = DictionaryConfig {
        kind: DictionaryKind::IPADIC,
        path: Some(PathBuf::from(
            "./src/resource/lindera-ipadic-2.7.0-20070801-neologd-20200910",
        )),
    };

    let config = TokenizerConfig {
        dictionary,
        mode: Mode::Normal,
        user_dictionary: None,
    };

    config
}

#[cfg(test)]
mod test {
    use crate::domain::Noun;

    use super::*;

    #[test]
    #[mry::lock(setup_ipadic_neologd)]
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

        mock_setup_ipadic_neologd().returns(config);

        let actual = aggregate_group_by_noun(word).unwrap();
        assert_eq!(actual, expected)
    }
}
