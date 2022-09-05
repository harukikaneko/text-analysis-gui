use itertools::Itertools;

use super::{Noun, Nouns};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
    pub fn exclude_non_nouns(self) -> Nouns {
        Nouns(
            self.0
                .into_iter()
                .filter(|x| Detail(x.detail.clone()).is_nouns())
                .map(|nouns| Noun(nouns.text))
                .collect_vec(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub detail: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detail(Vec<String>);

impl Detail {
    pub fn is_nouns(&self) -> bool {
        self.0.contains(&"名詞".to_string()) || self.0.contains(&"カスタム名詞".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exclude_non_nouns() {
        let token = Token {
            text: "東京スカイツリー".into(),
            detail: vec!["名詞".into()],
        };
        let exclude_token = Token {
            text: "の".into(),
            detail: vec!["助詞".into()],
        };
        let tokens = Tokens(vec![token, exclude_token]);
        let expected = Nouns(vec![Noun("東京スカイツリー".into())]);
        assert_eq!(tokens.exclude_non_nouns(), expected)
    }

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
}
