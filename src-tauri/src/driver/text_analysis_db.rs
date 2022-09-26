use crate::config::DB_POOL;

#[mry::mry]
pub async fn bulk_insert(year: usize, nouns: Vec<String>) -> anyhow::Result<()> {
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<String> = Vec::new();
    nouns.into_iter().for_each(|i| {
        v1.push(year as i64);
        v2.push(i);
    });

    let mut tx = DB_POOL.get().unwrap().begin().await?;

    sqlx::query(
        r#"
            INSERT INTO nouns (year, noun)
            SELECT * FROM UNNEST($1, $2)
            RETURNING year, noun
        "#,
    )
    .bind(&v1)
    .bind(&v2)
    .execute(&mut tx)
    .await
    .map_err(|e| anyhow::anyhow!("error insert {:?}", e))?;

    tx.commit().await?;

    Ok(())
}
