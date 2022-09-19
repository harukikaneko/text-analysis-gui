use futures::future::try_join_all;
use itertools::Itertools;

use crate::{domain::TextWithYears, gateway};

pub async fn texts_translate(texts: TextWithYears) -> anyhow::Result<TextWithYears> {
    let filter_en = texts.clone().filter_en_abstract();
    if filter_en.is_exists_en_abstract() {
        let handles = filter_en
            .0
            .into_iter()
            .map(|v| gateway::translate::translate_en_text(v.text, v.year))
            .collect_vec();

        let translate_text = TextWithYears(try_join_all(handles).await?);
        return Ok(texts.exclude_en_abstract().push_items(translate_text));
    }
    Ok(texts)
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{Text, TextWithYear},
        gateway::translate::{mock_translate_en_text, translate_en_text},
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(translate_en_text)]
    async fn test_texts_translate_include_en_text() {
        let expected = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                text: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                text: Text("Advanced DCBS特有の課題:リソースの組織化".into()),
            },
        ]);

        let texts = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                text: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                text: Text("Advanced DCBSpecific Challenge:The organisation of resources".into()),
            },
        ]);

        let translated = TextWithYear {
            year: 2022,
            text: Text("Advanced DCBS特有の課題:リソースの組織化".into()),
        };

        mock_translate_en_text(
            Text("Advanced DCBSpecific Challenge:The organisation of resources".into()),
            2022,
        )
        .returns_with(move |_, _| Ok(translated.clone()));

        let actual = texts_translate(texts).await.unwrap();
        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_texts_translate_not_include_en_text() {
        let expected = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                text: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                text: Text("Advanced DCBS特有の課題:リソースの組織化".into()),
            },
        ]);

        let texts = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                text: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                text: Text("Advanced DCBS特有の課題:リソースの組織化".into()),
            },
        ]);

        let actual = texts_translate(texts).await.unwrap();
        assert_eq!(expected, actual)
    }
}
