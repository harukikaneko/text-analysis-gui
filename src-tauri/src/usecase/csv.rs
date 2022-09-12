use std::fs;

use crate::domain::TextWithYears;

pub async fn read_csv(file_path: String) -> anyhow::Result<TextWithYears> {
    let mut csv_list = Vec::new();

    let csv_text = fs::read_to_string(file_path)?;
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        let record = result?.deserialize(None)?;
        csv_list.push(record);
    }

    Ok(csv_list.into())
}
