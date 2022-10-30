use std::env;

use dotenv::dotenv;
use migration::{database_config::DatabaseConfiguration, init_db, Db};

pub async fn migrate_and_init_db() -> Result<Db, sqlx::Error> {
    dotenv().unwrap();
    let pg_host = env::var("PG_HOST").unwrap();
    let pg_db = env::var("PG_DB").unwrap();
    let pg_user = env::var("PG_USER").unwrap();
    let pg_password = env::var("PG_PASSWORD").unwrap();

    let config = DatabaseConfiguration::new(
        pg_host.as_str(),
        pg_db.as_str(),
        pg_user.as_str(),
        pg_password.as_str(),
        5,
    );

    let pool = init_db(&config).await?;

    sqlx::migrate!("./db/migration")
        .run(&pool)
        .await?;

    Ok(pool)
}
