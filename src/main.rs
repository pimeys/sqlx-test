use bytes::BytesMut;
use sqlx::{
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgConnection, PgTypeInfo},
    Connection, Done, Encode, Postgres, Type,
};
use tokio_postgres::types::{IsNull as IsNull2, ToSql, Type as PostgresType};

const STMT: &'static str = "INSERT INTO \"Cat\" (id,mood) VALUES ($1,$2)";
const CONN_STR: &'static str = "postgres://postgres:prisma@localhost:5432/postgres";

#[derive(Debug)]
struct EnumString(String);

impl ToSql for EnumString {
    fn to_sql(
        &self,
        _: &PostgresType,
        out: &mut BytesMut,
    ) -> Result<IsNull2, Box<dyn std::error::Error + 'static + Send + Sync>> {
        out.extend_from_slice(self.0.as_bytes());
        Ok(IsNull2::No)
    }

    fn accepts(_: &PostgresType) -> bool {
        true
    }

    tokio_postgres::types::to_sql_checked!();
}

impl Encode<'_, Postgres> for EnumString {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        <String as Encode<'_, Postgres>>::encode_by_ref(&self.0, buf)
    }
}

impl Type<Postgres> for EnumString {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(25) // TEXT
    }

    fn compatible(_: &PgTypeInfo) -> bool {
        true
    }
}

async fn sqlx_test() -> anyhow::Result<()> {
    let mut conn = PgConnection::connect(CONN_STR).await?;

    let done = sqlx::query(STMT)
        // this works as the underlying column is text
        .bind("asdf")
        // this will crash due to the column being an enum and we can't write it
        // as a string.
        .bind(EnumString("HUNGRY".into()))
        .execute(&mut conn)
        .await?;

    assert_eq!(1, done.rows_affected());

    Ok(())
}

async fn tokio_postgres_test() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect(CONN_STR, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let changes = client.execute(STMT, &[&"asdf", &EnumString("HUNGRY".into())]).await?;

    assert_eq!(1, changes);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio_postgres_test().await?;
    sqlx_test().await?;

    Ok(())
}
