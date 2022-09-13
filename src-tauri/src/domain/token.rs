use itertools::Itertools;

use super::{CountsOfNounsByYear, Noun, Nouns, NounsByYear};

#[derive(Debug, Clone, PartialEq)]
pub struct TokensWithYear(Vec<(usize, Tokens)>);

impl From<Vec<(usize, Tokens)>> for TokensWithYear {
    fn from(from: Vec<(usize, Tokens)>) -> Self {
        TokensWithYear(from)
    }
}

impl TokensWithYear {
    pub fn aggregate_exclude_outside_condition_by_year(self) -> Vec<CountsOfNounsByYear> {
        self.0
            .into_iter()
            .map(|item| CountsOfNounsByYear {
                year: item.0,
                nouns: item.1.exclude_outside_condition().aggregate_group_by_word(),
            })
            .collect_vec()
    }

    pub fn exclude_outside_condition_by_year(self) -> Vec<NounsByYear> {
        self.0
            .into_iter()
            .map(|item| NounsByYear {
                year: item.0,
                nouns: item.1.exclude_outside_condition(),
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tokens_with_year_test {
    use crate::domain::CountsByNoun;

    use super::*;

    #[test]
    fn test_from() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let target = vec![
            (2022, Tokens(vec![token.clone(), exclude_token.clone()])),
            (2021, Tokens(vec![token, exclude_token])),
        ];

        let actual: TokensWithYear = target.clone().into();
        assert_eq!(actual, TokensWithYear(target))
    }

    #[test]
    fn test_aggregate_exclude_outside_condition_by_year() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let target = TokensWithYear(vec![
            (2022, Tokens(vec![token.clone(), exclude_token.clone()])),
            (2021, Tokens(vec![token, exclude_token])),
        ]);

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
        assert_eq!(
            target.aggregate_exclude_outside_condition_by_year(),
            expected
        )
    }

    #[test]
    fn exclude_outside_condition_by_year() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };

        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };

        let target = TokensWithYear(vec![
            (2022, Tokens(vec![token.clone(), exclude_token.clone()])),
            (2021, Tokens(vec![token, exclude_token])),
        ]);

        let expected = vec![
            NounsByYear {
                year: 2022,
                nouns: Nouns(vec![Noun("東京スカイツリー".into())]),
            },
            NounsByYear {
                year: 2021,
                nouns: Nouns(vec![Noun("東京スカイツリー".into())]),
            },
        ];
        assert_eq!(target.exclude_outside_condition_by_year(), expected)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
    pub fn exclude_outside_condition(self) -> Nouns {
        Nouns(
            self.0
                .into_iter()
                .filter(|x| {
                    x.word.limit_word_length(2)
                        && x.detail.is_nouns()
                        && x.detail.not_number()
                        && x.detail.not_pronouns()
                        && x.detail.not_independent()
                        && x.detail.not_adjectival_stem()
                        && x.detail.not_adverbable()
                })
                .map(|nouns| Noun(nouns.word.0))
                .collect_vec(),
        )
    }
}

#[cfg(test)]
mod tokens_test {
    use super::*;

    #[test]
    fn test_exclude_non_unconditional() {
        let token = Token {
            word: Word("東京スカイツリー".into()),
            detail: Detail(vec!["名詞".into()]),
        };
        let exclude_token = Token {
            word: Word("の".into()),
            detail: Detail(vec!["助詞".into()]),
        };
        let tokens = Tokens(vec![token, exclude_token]);
        let expected = Nouns(vec![Noun("東京スカイツリー".into())]);
        assert_eq!(tokens.exclude_outside_condition(), expected)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub word: Word,
    pub detail: Detail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detail(pub Vec<String>);

impl Detail {
    pub fn is_nouns(&self) -> bool {
        self.0.contains(&"名詞".to_string()) || self.0.contains(&"カスタム名詞".to_string())
    }

    pub fn not_number(&self) -> bool {
        !self.0.contains(&"数".to_string())
    }

    pub fn not_independent(&self) -> bool {
        !self.0.contains(&"非自立".to_string())
    }

    pub fn not_pronouns(&self) -> bool {
        !self.0.contains(&"代名詞".to_string())
    }

    pub fn not_adjectival_stem(&self) -> bool {
        !self.0.contains(&"形容動詞語幹".to_string())
    }

    pub fn not_adverbable(&self) -> bool {
        !self.0.contains(&"副詞可能".to_string())
    }
}

#[cfg(test)]
mod detail_test {
    use super::*;

    #[test]
    fn test_is_nouns() {
        let detail = Detail(vec!["名詞".into()]);
        assert!(detail.is_nouns())
    }

    #[test]
    fn test_is_not_nouns() {
        let detail = Detail(vec!["助詞".into()]);
        assert!(!detail.is_nouns())
    }

    #[test]
    fn test_is_number() {
        let detail = Detail(vec![
            "名詞".into(),
            "数".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "１".into(),
            "イチ".into(),
            "イチ".into(),
        ]);
        assert!(!detail.not_number())
    }

    #[test]
    fn test_is_not_number() {
        let detail = Detail(vec!["助詞".into()]);
        assert!(detail.not_number())
    }

    #[test]
    fn test_is_independent() {
        let detail = Detail(vec![
            "名詞".into(),
            "非自立".into(),
            "副詞可能".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "ため".into(),
            "タメ".into(),
            "タメ".into(),
        ]);
        assert!(!detail.not_independent())
    }

    #[test]
    fn test_is_not_independent() {
        let detail = Detail(vec!["名詞".into(), "代名詞".into(), "一般".into()]);
        assert!(detail.not_independent())
    }

    #[test]
    fn test_is_pronouns() {
        let detail = Detail(vec![
            "名詞".into(),
            "代名詞".into(),
            "一般".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "これら".into(),
            "コレラ".into(),
            "コレラ".into(),
        ]);
        assert!(!detail.not_pronouns())
    }

    #[test]
    fn test_is_not_pronouns() {
        let detail = Detail(vec!["名詞".into(), "非自立".into(), "副詞可能".into()]);
        assert!(detail.not_pronouns())
    }

    #[test]
    fn test_is_adjectival_stem() {
        let detail = Detail(vec![
            "名詞".into(),
            "形容動詞語幹".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "*".into(),
            "明らか".into(),
            "アキラカ".into(),
            "アキラカ".into(),
        ]);
        assert!(!detail.not_adjectival_stem())
    }

    #[test]
    fn test_is_not_adjectival_stem() {
        let detail = Detail(vec!["名詞".into(), "非自立".into(), "副詞可能".into()]);
        assert!(detail.not_adjectival_stem())
    }

    #[test]
    fn test_is_adverbable() {
        let detail = Detail(vec![
            "名詞".into(),
            "副詞可能".into(),
            "すべて".into(),
            "スベテ".into(),
            "スベテ".into(),
        ]);
        assert!(!detail.not_adverbable())
    }

    #[test]
    fn test_is_not_adverbable() {
        let detail = Detail(vec!["名詞".into(), "非自立".into()]);
        assert!(detail.not_adverbable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(pub String);

impl Word {
    pub fn limit_word_length(&self, limit: usize) -> bool {
        self.0.chars().count() > limit
    }
}

#[cfg(test)]
mod word_test {
    use super::*;

    #[test]
    fn test_limit_word_length_2_gt() {
        let target = Word("東京スカイツリー".into());
        assert!(target.limit_word_length(2))
    }

    #[test]
    fn test_limit_word_length_2_lt() {
        let target = Word("東京".into());
        assert!(!target.limit_word_length(2))
    }
}
