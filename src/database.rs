use sqlx::sqlite::{SqlitePool};

pub(crate) async fn lists(pool: &SqlitePool) -> anyhow::Result<()> {
    let records = sqlx::query!(
        r#"
            SELECT id, description FROM passwords ORDER BY id
        "#
    ).fetch_all(pool)
    .await?;

    for record in records {
        println!(
            "{} | {:?}",
            record.id,
            record.description,
        );
    }

    Ok(())
}

pub(crate) async fn add(pool: &SqlitePool, password: &str, description: &str) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;
    let id = sqlx::query!(
        r#"
            INSERT INTO passwords (password, description) VALUES(?, ?)
        "#,
        password,
        description
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub(crate) async fn delete(pool: &SqlitePool, description: &str) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;
    let _ = sqlx::query!(
        r#"
            DELETE FROM passwords WHERE description = ?
        "#,
        description,
    ).execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(())
}
