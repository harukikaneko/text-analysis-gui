use futures::future::try_join_all;
use itertools::Itertools;

use crate::{domain::NounsByYear, driver};

#[mry::mry]
pub async fn register_nouns_by_year(create_target: Vec<NounsByYear>) -> anyhow::Result<()> {
    let handles = create_target
        .into_iter()
        .map(|i| {
            driver::text_analysis_db::bulk_insert(
                i.year,
                i.nouns.0.into_iter().map(|noun| noun.0).collect_vec(),
            )
        })
        .collect_vec();

    try_join_all(handles).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{Noun, Nouns},
        driver::text_analysis_db::{bulk_insert, mock_bulk_insert},
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(bulk_insert)]
    async fn test_register_nouns_by_year() {
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

        mock_bulk_insert(2022, vec!["東京スカイツリー".into()]).returns_with(move |_, _| Ok(()));
        mock_bulk_insert(2021, vec!["東京スカイツリー".into()]).returns_with(move |_, _| Ok(()));

        assert!(register_nouns_by_year(create_target).await.is_ok())
    }
}
