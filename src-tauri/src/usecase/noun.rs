use crate::{
    domain::{CountsByNoun, CountsOfNounsByYear, NounsByYear, Tokens},
    gateway,
};
use itertools::Itertools;
use lindera::LinderaResult;

pub fn aggregate_group_by_noun(tokens: Tokens) -> LinderaResult<Vec<CountsByNoun>> {
    let exclude_non_nouns = tokens.exclude_non_unconditional();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

pub fn aggregate_counts_of_nouns_by_year(
    aggregate_target: Vec<(usize, Tokens)>,
) -> LinderaResult<Vec<CountsOfNounsByYear>> {
    // move domain logic
    Ok(aggregate_target
        .into_iter()
        .map(|item| CountsOfNounsByYear {
            year: item.0,
            nouns: item.1.exclude_non_unconditional().aggregate_group_by_word(),
        })
        .collect_vec())
}

pub async fn create_of_nouns_by_year(target: Vec<(usize, Tokens)>) -> anyhow::Result<()> {
    // move domain logic
    let create_target = target
        .into_iter()
        .map(|item| NounsByYear {
            year: item.0,
            nouns: item.1.exclude_non_unconditional(),
        })
        .collect_vec();
    gateway::noun::register_nouns_by_year(create_target).await
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{Detail, Noun, Nouns, Token, Word},
        gateway::noun::{mock_register_nouns_by_year, register_nouns_by_year},
    };

    use super::*;

    #[test]
    fn test_aggregate_group_by_noun() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let tokens = Tokens(vec![token, exclude_token]);

        let expected = vec![CountsByNoun {
            noun: Noun("東京スカイツリー".into()),
            counts: 1,
        }];

        let actual = aggregate_group_by_noun(tokens).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_aggregate_counts_of_nouns_by_year() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let tokens = vec![
            (2022, Tokens(vec![token.clone(), exclude_token.clone()])),
            (2021, Tokens(vec![token, exclude_token])),
        ];

        let expected = vec![
            CountsOfNounsByYear {
                year: 2022,
                nouns: vec![CountsByNoun {
                    noun: Noun("東京スカイツリー".into()),
                    counts: 1,
                }],
            },
            CountsOfNounsByYear {
                year: 2021,
                nouns: vec![CountsByNoun {
                    noun: Noun("東京スカイツリー".into()),
                    counts: 1,
                }],
            },
        ];

        let actual = aggregate_counts_of_nouns_by_year(tokens).unwrap();
        assert_eq!(actual, expected)
    }

    #[tokio::test]
    #[mry::lock(register_nouns_by_year)]
    async fn test_create_of_nouns_by_year() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let tokens = vec![
            (2022, Tokens(vec![token.clone(), exclude_token.clone()])),
            (2021, Tokens(vec![token, exclude_token])),
        ];

        let create_target = vec![
            NounsByYear {
                year: 2022,
                nouns: Nouns(vec![Noun("東京スカイツリー".into())]),
            },
            NounsByYear {
                year: 2021,
                nouns: Nouns(vec![Noun("東京スカイツリー".into())]),
            },
        ];

        mock_register_nouns_by_year(create_target.clone()).returns_with(move |_| Ok(()));
        assert!(create_of_nouns_by_year(tokens).await.is_ok())
    }
}
