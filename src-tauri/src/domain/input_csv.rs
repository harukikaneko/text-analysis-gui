use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TextWithYears(pub Vec<TextWithYear>);

impl From<Vec<TextWithYear>> for TextWithYears {
    fn from(from: Vec<TextWithYear>) -> Self {
        TextWithYears(from)
    }
}

impl TextWithYears {
    pub fn group_by_year(self) -> Self {
        Self(
            self.0
                .into_iter()
                .into_group_map_by(|target| target.year)
                .into_iter()
                .map(|group| TextWithYear {
                    year: group.0,
                    r#abstract: Text(join_text(group.1)),
                })
                .collect_vec(),
        )
    }

    pub fn filter_en_abstract(self) -> Self {
        Self(
            self.0
                .into_iter()
                .filter(|v| v.r#abstract.is_ascii_alphabet())
                .collect_vec(),
        )
    }

    pub fn exclude_en_abstract(self) -> Self {
        Self(
            self.0
                .into_iter()
                .filter(|v| !v.r#abstract.is_ascii_alphabet())
                .collect_vec(),
        )
    }

    pub fn push_items(mut self, items: TextWithYears) -> Self {
        items.0.into_iter().for_each(|item| {
            self.0.push(item);
            println!("{:?}", self);
        });
        println!("{:?}", self);
        self
    }

    pub fn is_exists_en_abstract(&self) -> bool {
        !self.0.is_empty()
    }
}

fn join_text(array: Vec<TextWithYear>) -> String {
    array
        .into_iter()
        .map(|i| i.r#abstract.0)
        .collect_vec()
        .join("")
}

#[cfg(test)]
mod text_years_test {
    use super::*;

    #[test]
    fn test_group_by_year() {
        let expected = TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("東京スカイツリー東京".into()),
        }]);

        let target = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京".into()),
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
                r#abstract: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京".into()),
            },
        ];

        let actual = join_text(target);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_filter_en_abstract() {
        let target = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                r#abstract: Text(
                    "Advanced DCBSpecific Challenge:The organisation of resources".into(),
                ),
            },
        ]);

        let expected = TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("Advanced DCBSpecific Challenge:The organisation of resources".into()),
        }]);

        assert_eq!(target.filter_en_abstract(), expected)
    }

    #[test]
    fn test_exclude_en_abstract() {
        let target = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                r#abstract: Text(
                    "Advanced DCBSpecific Challenge:The organisation of resources".into(),
                ),
            },
        ]);

        let expected = TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("東京スカイツリー".into()),
        }]);

        assert_eq!(target.exclude_en_abstract(), expected)
    }

    #[test]
    fn test_is_exists_en_abstract() {
        assert!(TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("Advanced DCBSpecific Challenge:The organisation of resources".into()),
        }])
        .is_exists_en_abstract())
    }

    #[test]
    fn test_not_exists_en_abstract() {
        assert!(!TextWithYears(vec![]).is_exists_en_abstract())
    }

    #[test]
    fn test_push_items() {
        let expected = TextWithYears(vec![
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京スカイツリー".into()),
            },
            TextWithYear {
                year: 2022,
                r#abstract: Text("東京".into()),
            },
        ]);

        let target = TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("東京スカイツリー".into()),
        }]);

        let input_items = TextWithYears(vec![TextWithYear {
            year: 2022,
            r#abstract: Text("東京".into()),
        }]);

        assert_eq!(target.push_items(input_items), expected)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TextWithYear {
    pub year: usize,
    pub r#abstract: Text,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Text(pub String);

impl Text {
    pub fn is_ascii_alphabet(&self) -> bool {
        self.0.is_ascii()
    }
}

#[cfg(test)]
mod text_with_year_test {
    use super::*;

    #[test]
    fn test_not_ascii_alphabet() {
        let noun = Text("東京スカイツリー".into());

        assert!(!noun.is_ascii_alphabet())
    }

    #[test]
    fn test_is_ascii_alphabet() {
        let noun = Text("Advanced DCBSpecific Challenge:The organisation of resources".into());

        assert!(noun.is_ascii_alphabet())
    }
}
