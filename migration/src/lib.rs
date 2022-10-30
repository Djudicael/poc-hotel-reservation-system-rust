use std::time::Duration;

use database_config::DatabaseConfiguration;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod database_config;

pub type Db = Pool<Postgres>;

pub async fn init_db(config: &DatabaseConfiguration) -> Result<Db, sqlx::Error> {
    let con_string = format!(
        "postgres://{}:{}@{}/{}",
        &config.pg_user.as_str(),
        &config.pg_password.as_str(),
        &config.pg_host.as_str(),
        &config.pg_db.as_str()
    );

    PgPoolOptions::new()
        .max_connections(config.pg_app_max_con)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&con_string)
        .await
}
