use sqlx::{
    encode::{Encode, IsNull},
    postgres::{PgArgumentBuffer, PgConnection, PgRow, PgTypeInfo},
    Connect, Postgres, Row, Type,
};

#[derive(Debug)]
enum Value {
    Integer(Option<i64>),
    Float(Option<f64>),
    Array(Option<Vec<Value>>),
}

impl<'a> Encode<'a, Postgres> for Value {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Self::Integer(opt_i) => <Option<i64> as Encode<'a, Postgres>>::encode(*opt_i, buf),
            Self::Float(opt_f) => <Option<f64> as Encode<'a, Postgres>>::encode(*opt_f, buf),
            // Self::Array(opt_a) => <Option<Vec<Value>> as Encode<'a, Postgres>>::encode_by_ref(opt_a, buf),
            Self::Array(_opt_a) => todo!("The thing above ^^ doesn't work. What kind of type trick we need?"),
        }
    }

    fn produces(&self) -> Option<PgTypeInfo> {
        match self {
            Self::Integer(_) => Some(<i64 as Type<Postgres>>::type_info()),
            Self::Float(_) => Some(<f64 as Type<Postgres>>::type_info()),
            Self::Array(_) => todo!("???"),
        }
    }
}

impl Type<Postgres> for Value {
    fn type_info() -> PgTypeInfo {
        unreachable!()
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = PgConnection::connect("postgres://postgres:password@localhost:5432/sqlx").await?;

    let rows: Vec<(Value, Value, Value)> = sqlx::query("SELECT $1, $2, $3")
        .bind(Value::Integer(Some(12)))
        .bind(Value::Float(Some(4.2)))
        .bind(Value::Array(Some(vec![Value::Integer(Some(12))])))
        .try_map(|row: PgRow| {
            Ok((
                Value::Integer(row.get(0)),
                Value::Float(row.get(1)),
                Value::Array(todo!("and how to read this back?")),
            ))
        })
        .fetch_all(&mut conn)
        .await?;

    dbg!(rows);

    Ok(())
}
