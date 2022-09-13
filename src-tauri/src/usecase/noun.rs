use crate::{
    domain::{CountsByNoun, CountsOfNounsByYear, Tokens, TokensWithYear},
    gateway,
};
use lindera::LinderaResult;

pub fn aggregate_group_by_noun(tokens: Tokens) -> LinderaResult<Vec<CountsByNoun>> {
    let exclude_non_nouns = tokens.exclude_outside_condition();
    Ok(exclude_non_nouns.aggregate_group_by_word())
}

pub fn aggregate_counts_of_nouns_by_year(
    aggregate_target: Vec<(usize, Tokens)>,
) -> LinderaResult<Vec<CountsOfNounsByYear>> {
    let target: TokensWithYear = aggregate_target.into();
    Ok(target.aggregate_exclude_outside_condition_by_year())
}

pub async fn create_of_nouns_by_year(target: Vec<(usize, Tokens)>) -> anyhow::Result<()> {
    let create_target: TokensWithYear = target.into();
    gateway::noun::register_nouns_by_year(create_target.exclude_outside_condition_by_year()).await
}

#[cfg(test)]
mod test {
    use crate::gateway::noun::{mock_register_nouns_by_year, register_nouns_by_year};

    use super::*;

    #[test]
    fn test_aggregate_group_by_noun() {
        let tokens = Tokens(vec![]);

        let expected = vec![];

        let actual = aggregate_group_by_noun(tokens).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_aggregate_counts_of_nouns_by_year() {
        let tokens = vec![];

        let expected = vec![];

        let actual = aggregate_counts_of_nouns_by_year(tokens).unwrap();
        assert_eq!(actual, expected)
    }

    #[tokio::test]
    #[mry::lock(register_nouns_by_year)]
    async fn test_create_of_nouns_by_year() {
        let tokens = vec![];

        let create_target = vec![];

        mock_register_nouns_by_year(create_target.clone()).returns_with(move |_| Ok(()));
        assert!(create_of_nouns_by_year(tokens).await.is_ok())
    }
}
