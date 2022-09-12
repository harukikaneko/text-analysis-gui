use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nouns(pub Vec<Noun>);

impl Nouns {
    pub fn aggregate_group_by_word(self) -> Vec<CountsByNoun> {
        self.0
            .into_iter()
            .into_group_map_by(|word| word.to_owned())
            .into_iter()
            .map(|(key, values)| -> CountsByNoun {
                CountsByNoun {
                    noun: key,
                    counts: values.len(),
                }
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod nouns_test {
    use super::*;

    #[test]
    fn test_aggregate_group_by_word() {
        let nouns = Nouns(vec![
            Noun("東京スカイツリー".into()),
            Noun("東京スカイツリー".into()),
        ]);

        let expected = vec![CountsByNoun {
            noun: Noun("東京スカイツリー".into()),
            counts: 2,
        }];
        assert_eq!(nouns.aggregate_group_by_word(), expected)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, Eq)]
pub struct Noun(pub String);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CountsByNoun {
    pub noun: Noun,
    pub counts: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CountsOfNounsByYear {
    pub year: usize,
    pub nouns: Vec<CountsByNoun>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounsByYear {
    pub year: usize,
    pub nouns: Nouns,
}
