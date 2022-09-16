use itertools::Itertools;
use lindera::{tokenizer::Tokenizer, LinderaResult};

use crate::{
    config::dictionary_setup,
    domain::{Detail, Token, Tokens, Word},
};

pub fn get_tokens(
    word: String,
    dictionary_path: Option<String>,
    user_dictionary: Option<String>,
) -> LinderaResult<Tokens> {
    let config = dictionary_setup(dictionary_path, user_dictionary);
    let tokenizer = Tokenizer::with_config(config)?;

    let tokens = tokenizer
        .tokenize(&word)?
        .into_iter()
        .map(|token| Token {
            word: Word(token.text.to_string()),
            detail: Detail(token.detail),
        })
        .collect_vec(); // 元のstructを&str→Stringにしたい為,domainを詰め直している
    Ok(Tokens(tokens))
}

#[mry::mry]
pub async fn get_tokens_by_year(
    year: usize,
    word: String,
    dictionary_path: &Option<String>,
    user_dictionary: &Option<String>,
) -> anyhow::Result<(usize, Tokens)> {
    let tokens = get_tokens(word, dictionary_path.to_owned(), user_dictionary.to_owned())?;
    Ok((year, tokens))
}

#[cfg(test)]
mod test {
    use lindera::{
        mode::Mode,
        tokenizer::{DictionaryConfig, DictionaryKind, TokenizerConfig},
    };

    use crate::config::mock_dictionary_setup;

    use super::*;

    #[test]
    #[mry::lock(dictionary_setup)]
    fn test_get_tokens() {
        let word = "東京の".into();
        let dictionary = DictionaryConfig {
            kind: DictionaryKind::IPADIC,
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            mode: Mode::Normal,
            user_dictionary: None,
        };

        let token = Token {
            word: Word("東京".into()),
            detail: Detail(vec![
                "名詞".into(),
                "固有名詞".into(),
                "地域".into(),
                "一般".into(),
                "*".into(),
                "*".into(),
                "東京".into(),
                "トウキョウ".into(),
                "トーキョー".into(),
            ]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec![
                "助詞".into(),
                "連体化".into(),
                "*".into(),
                "*".into(),
                "*".into(),
                "*".into(),
                "の".into(),
                "ノ".into(),
                "ノ".into(),
            ]),
        };

        let expected = Tokens(vec![token, exclude_token]);

        mock_dictionary_setup(None, None).returns(config);

        assert_eq!(get_tokens(word, None, None).unwrap(), expected)
    }

    #[tokio::test]
    #[mry::lock(dictionary_setup)]
    async fn test_get_tokens_by_year() {
        let year = 2022;
        let word = "東京の".into();
        let dictionary = DictionaryConfig {
            kind: DictionaryKind::IPADIC,
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            mode: Mode::Normal,
            user_dictionary: None,
        };

        let token = Token {
            word: Word("東京".into()),
            detail: Detail(vec![
                "名詞".into(),
                "固有名詞".into(),
                "地域".into(),
                "一般".into(),
                "*".into(),
                "*".into(),
                "東京".into(),
                "トウキョウ".into(),
                "トーキョー".into(),
            ]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec![
                "助詞".into(),
                "連体化".into(),
                "*".into(),
                "*".into(),
                "*".into(),
                "*".into(),
                "の".into(),
                "ノ".into(),
                "ノ".into(),
            ]),
        };

        let expected = (year, Tokens(vec![token, exclude_token]));

        mock_dictionary_setup(None, None).returns(config);

        assert_eq!(
            get_tokens_by_year(year, word, &None, &None).await.unwrap(),
            expected
        )
    }
}
