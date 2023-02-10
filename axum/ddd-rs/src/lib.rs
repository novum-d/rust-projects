#[macro_use]
extern crate dotenv_codegen;

use std::any;

use log::info;
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};

struct Stores {
    city: String,
    hours: String,
    name: String,
    open: bool,
    phone_number: String,
    street: String,
    zip: String,
}

#[sqlx::test]
async fn it_connects() -> sqlx::Result<()> {
    let pool = PgPool::connect(dotenv!("DATABASE_URL")).await?;

    let stores = sqlx::query("SELECT * FROM stores")
        .map(|x: PgRow| {
            info!("column nu!!!!!!!!!!!!!!!!!!1{}", x.len());
            x
        })
        .fetch_all(&pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn dotenv_test() {
        assert_eq!(
            "postgres://postgres:postgres@localhost/lv_ex_dev",
            dotenv!("DATABASE_URL")
        )
    }
}
