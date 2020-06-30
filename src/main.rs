use sqlx::{
    mysql::{MySqlArguments, MySqlConnection, MySqlRow},
    query::Query,
    Connect, FromRow, MySql, Row,
};

enum Value {
    Integer(u64),
    Boolean(bool),
}

trait Bind {
    fn bind_value(self, value: Value) -> Self
    where
        Self: Sized;
}

impl<'a> Bind for Query<'a, MySql, MySqlArguments> {
    fn bind_value(self, value: Value) -> Self {
        match value {
            Value::Integer(i) => self.bind(i),
            Value::Boolean(b) => self.bind(b),
        }
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = MySqlConnection::connect("mysql://root:password@localhost:3306/sqlx").await?;

    let query = sqlx::query("SELECT ?").bind_value(Value::Integer(1));

    let rows: Vec<Value> = query.map(|row| Value::Integer(row.get(0))).fetch_all(&mut conn).await?;

    println!("Hello, world!");
    Ok(())
}
