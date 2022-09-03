use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Nouns(pub Vec<Noun>);

impl Nouns {
    pub fn aggregate_group_by_word(self) -> Vec<CountByNoun> {
        self.0
            .into_iter()
            .into_group_map_by(|word| word.to_owned())
            .into_iter()
            .map(|(key, values)| -> CountByNoun {
                CountByNoun {
                    noun: key,
                    counts: values.len(),
                }
            })
            .collect_vec()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, Eq)]
pub struct Noun(pub String);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CountByNoun {
    pub noun: Noun,
    pub counts: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aggregate_group_by_word() {
        let nouns = Nouns(vec![
            Noun("東京スカイツリー".into()),
            Noun("東京スカイツリー".into()),
        ]);

        let expected = vec![CountByNoun {
            noun: Noun("東京スカイツリー".into()),
            counts: 2,
        }];
        assert_eq!(nouns.aggregate_group_by_word(), expected)
    }
}
