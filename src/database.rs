use sqlx::sqlite::{SqlitePool};

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
