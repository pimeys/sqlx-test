use sqlx::{
    sqlite::{SqliteConnection, SqliteRow},
    Connection, Row, TypeInfo, Value, ValueRef,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = SqliteConnection::connect("sqlite:test.db").await?;

    let data: Vec<Option<i64>> = sqlx::query("PRAGMA table_info (\"Test\")")
        .try_map(
            |row: SqliteRow| match row.try_get_raw("dflt_value").unwrap().to_owned() {
                // This works, but is impossible to guess right due to the name
                // being always `NULL`.
                //
                // The underlying column is giving the default value for the
                // column it describes. This can be null or of any type.
                //
                // Here it's `Option<i64>` so we can use the `decode_unchecked`.
                // It gets trickier to cast when having some other type as the
                // default value.
                vr if vr.type_info().name() == "NULL" => Ok(vr.decode_unchecked()),
                vr => panic!(vr.type_info().name().to_string()),
            },
        )
        .fetch_all(&mut conn)
        .await?;

    dbg!(data);

    Ok(())
}
