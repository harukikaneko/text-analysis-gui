use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TextWithYears(pub Vec<TextWithYear>);

impl TextWithYears {
    pub fn group_by_year(self) -> Self {
        Self(
            self.0
                .into_iter()
                .into_group_map_by(|target| target.year)
                .into_iter()
                .map(|group| TextWithYear {
                    year: group.0,
                    r#abstract: join_text(group.1),
                })
                .collect_vec(),
        )
    }
}

fn join_text(array: Vec<TextWithYear>) -> String {
    array
        .into_iter()
        .map(|i| i.r#abstract)
        .collect_vec()
        .join("")
}

#[cfg(test)]
mod text_years_test {
    use super::*;

    #[test]
    fn test_group_by_year() {
        let expected = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: "東京スカイツリー東京".into(),
            },
            TextWithYear {
                year: 2021,
                r#abstract: "スカイツリー".into(),
            },
        ]);

        let target = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: "東京スカイツリー".into(),
            },
            TextWithYear {
                year: 2022,
                r#abstract: "東京".into(),
            },
            TextWithYear {
                year: 2021,
                r#abstract: "スカイツリー".into(),
            },
        ]);

        let actual = target.group_by_year();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_join_text() {
        let expected = "東京スカイツリー東京".to_string();
        let target = vec![
            TextWithYear {
                year: 2022,
                r#abstract: "東京スカイツリー".into(),
            },
            TextWithYear {
                year: 2022,
                r#abstract: "東京".into(),
            },
        ];

        let actual = join_text(target);
        assert_eq!(actual, expected)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TextWithYear {
    pub year: usize,
    pub r#abstract: String,
}
