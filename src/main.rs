use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection, Row,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite:test.db").await?;

    let data: Vec<Option<i64>> = sqlx::query("PRAGMA table_info (\"Test\")")
        .try_map(|row: SqliteRow| Ok(row.get("dflt_value")))
        .fetch_all(&mut conn)
        .await?;

    dbg!(data);

    println!("Hello, world!");
    Ok(())
}
