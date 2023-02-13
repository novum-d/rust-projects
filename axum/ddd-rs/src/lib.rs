#[macro_use]
#[allow(unused_imports)]
extern crate dotenv_codegen;

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    #[test]
    fn read_db_env() {
        let database_url = "postgres://postgres:postgres@localhost/lv_ex_dev";
        assert_eq!(database_url, dotenv!("DATABASE_URL"))
    }

    #[sqlx::test]
    async fn query_all() -> sqlx::Result<()> {
        let pool = PgPool::connect(dotenv!("DATABASE_URL")).await?;
        sqlx::query_as::<_, Stores>("SELECT * FROM stores")
            .fetch_all(&pool)
            .await?;
        Ok(())
    }

    #[derive(sqlx::FromRow, Debug)]
    #[allow(dead_code)]
    pub struct Stores {
        city: String,
        hours: String,
        name: String,
        open: bool,
        phone_number: String,
        street: String,
        zip: String,
    }
}
