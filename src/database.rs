use sqlx::sqlite::{SqlitePool};

pub(crate) async fn lists(pool: &SqlitePool) -> anyhow::Result<()> {
    let records = sqlx::query!(
        r#"
            SELECT id, key, password FROM passwords ORDER BY id
        "#
    ).fetch_all(pool)
    .await?;

    for record in records {
        println!(
            "{} | {} | {}",
            record.id,
            record.key,
            record.password,
        );
    }

    Ok(())
}

pub(crate) async fn add(pool: &SqlitePool, key: &str, password: &str) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
            INSERT INTO passwords (key, password) VALUES(?, ?)
        "#,
        key,
        password,
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub(crate) async fn delete(pool: &SqlitePool, key: &str) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let _ = sqlx::query!(
        r#"
            DELETE FROM passwords WHERE key = ?
        "#,
        key,
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(())
}
