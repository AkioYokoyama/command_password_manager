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
            "{:<02} | {:<8} | {:*<8}",
            record.id,
            record.key,
            record.password.chars().nth(0).unwrap(),
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

pub(crate) async fn find_by_key(pool: &SqlitePool, key: &str) -> anyhow::Result<String> {
    let record = sqlx::query!(
        r#"
            SELECT password FROM passwords WHERE key = ?
        "#,
        key
    ).fetch_one(pool)
    .await?;

    Ok(record.password)
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
