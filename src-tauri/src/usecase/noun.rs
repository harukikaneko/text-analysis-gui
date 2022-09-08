use crate::domain::{CountsByNoun, CountsOfNounsByYear, Tokens};
use itertools::Itertools;
use lindera::LinderaResult;

pub fn aggregate_group_by_noun(tokens: Tokens) -> LinderaResult<Vec<CountsByNoun>> {
    let exclude_non_nouns = tokens.exclude_non_nouns();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

pub fn aggregate_counts_of_nouns_by_year(
    aggregate_target: Vec<(usize, Tokens)>,
) -> LinderaResult<Vec<CountsOfNounsByYear>> {
    Ok(aggregate_target
        .into_iter()
        .map(|item| CountsOfNounsByYear {
            year: item.0,
            nouns: item.1.exclude_non_nouns().aggregate_group_by_word(),
        })
        .collect_vec())
}

#[cfg(test)]
mod test {
    use crate::domain::{Detail, Noun, Token, Word};

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
}
