use sqlx::{
    encode::{Encode, IsNull},
    postgres::{PgArgumentBuffer, PgConnection, PgRow, PgTypeInfo},
    Connect, Postgres, Row, Type,
};

#[derive(Debug, Clone)]
enum Value {
    Integer(Option<i64>),
    Float(Option<f64>),
    Array(Option<Vec<Value>>),
}

impl Value {
    fn to_int(self) -> Option<i64> {
        match self {
            Self::Integer(i) => i,
            _ => panic!("wrong"),
        }
    }

    fn to_float(self) -> Option<f64> {
        match self {
            Self::Float(f) => f,
            _ => panic!("wrong"),
        }
    }
}

impl<'a> Encode<'a, Postgres> for Value {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Self::Integer(opt_i) => <Option<i64> as Encode<'a, Postgres>>::encode(*opt_i, buf),
            Self::Float(opt_f) => <Option<f64> as Encode<'a, Postgres>>::encode(*opt_f, buf),
            Self::Array(Some(ary)) => match ary.first() {
                Some(Value::Integer(_)) => {
                    let ints = ary.into_iter().map(|i| i.clone().to_int()).collect();
                    <Vec<Option<i64>> as Encode<'a, Postgres>>::encode(ints, buf)
                }
                Some(Value::Float(_)) => {
                    let floats = ary.into_iter().map(|f| f.clone().to_float()).collect();
                    <Vec<Option<f64>> as Encode<'a, Postgres>>::encode(floats, buf)
                }
                Some(Value::Array(_)) => unreachable!(),
                None => <Vec<Option<f64>> as Encode<'a, Postgres>>::encode(vec![], buf),
            },
            Self::Array(None) => <Vec<Option<f64>> as Encode<'a, Postgres>>::encode(vec![], buf),
        }
    }

    fn produces(&self) -> Option<PgTypeInfo> {
        match self {
            Self::Integer(_) => Some(<i64 as Type<Postgres>>::type_info()),
            Self::Float(_) => Some(<f64 as Type<Postgres>>::type_info()),
            Self::Array(Some(ary)) => match ary.first() {
                Some(Value::Integer(_)) => Some(<Vec<i64> as Type<Postgres>>::type_info()),
                Some(Value::Float(_)) => Some(<Vec<f64> as Type<Postgres>>::type_info()),
                Some(Value::Array(_)) => unreachable!(),
                None => Some(<Vec<f64> as Type<Postgres>>::type_info()),
            },
            Self::Array(None) => Some(<Vec<f64> as Type<Postgres>>::type_info()),
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
