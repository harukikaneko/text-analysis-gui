use crate::{
    domain::{Text, TextWithYear},
    driver,
};

#[mry::mry]
pub async fn translate_en_text(text: Text, year: usize) -> anyhow::Result<TextWithYear> {
    let mut query = vec![("target_lang".into(), "JA".into())];
    query.push(("source_lang".into(), "EN".into()));
    query.push(("text".into(), text.0));

    let translated = driver::deepl_client::translate_to_ja(&query).await?;
    Ok(TextWithYear {
        year,
        text: Text(translated.translations[0].clone().text),
    })
}

#[cfg(test)]
mod test {
    use crate::driver::deepl_client::{
        mock_translate_to_ja, translate_to_ja, TranslatedText, TranslatedTextList,
    };

    use super::*;

    #[tokio::test]
    #[mry::lock(translate_to_ja)]
    async fn test_translate_en_text() {
        let expected = TextWithYear {
            year: 2022,
            text: Text("Advanced DCBS特有の課題:リソースの組織化".into()),
        };

        let query = vec![
            ("target_lang".into(), "JA".into()),
            ("source_lang".into(), "EN".into()),
            (
                "text".into(),
                "Advanced DCBSpecific Challenge:The organisation of resources".into(),
            ),
        ];

        mock_translate_to_ja(query).returns_with(move |_| {
            Ok(TranslatedTextList {
                translations: vec![TranslatedText {
                    detected_source_language: "EN".into(),
                    text: "Advanced DCBS特有の課題:リソースの組織化".into(),
                }],
            })
        });

        let actual = translate_en_text(
            Text("Advanced DCBSpecific Challenge:The organisation of resources".into()),
            2022,
        )
        .await
        .unwrap();

        assert_eq!(actual, expected)
    }
}
