use crate::domain::{CountsByNoun, CountsOfNounsByYear, TextWithYears, Tokens};
use lindera::LinderaResult;

pub fn aggregate_group_by_noun(tokens: Tokens) -> LinderaResult<Vec<CountsByNoun>> {
    let exclude_non_nouns = tokens.exclude_non_nouns();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

pub fn aggregate_counts_of_nouns_by_year(
    _aggregate_target: TextWithYears,
) -> LinderaResult<Vec<CountsOfNounsByYear>> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::domain::{Noun, Token};

    use super::*;

    #[test]
    fn test_aggregate_group_by_noun() {
        let token = Token {
            text: "東京スカイツリー".into(),
            detail: vec!["名詞".into()],
        };

        let exclude_token = Token {
            text: "の".into(),
            detail: vec!["助詞".into()],
        };

        let tokens = Tokens(vec![token, exclude_token]);

        let expected = vec![CountsByNoun {
            noun: Noun("東京スカイツリー".into()),
            counts: 1,
        }];

        let actual = aggregate_group_by_noun(tokens).unwrap();
        assert_eq!(actual, expected)
    }
}
