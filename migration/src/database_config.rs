#[derive(Debug)]
pub struct DatabaseConfiguration {
    pub pg_host: String,
    pub pg_db: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_app_max_con: u32,
}

impl DatabaseConfiguration {
    pub fn new(
        pg_host: &str,
        pg_db: &str,
        pg_user: &str,
        pg_password: &str,
        pg_app_max_con: u32,
    ) -> Self {
        Self {
            pg_host: pg_host.to_string(),
            pg_db: pg_db.to_string(),
            pg_user: pg_user.to_string(),
            pg_password: pg_password.to_string(),
            pg_app_max_con,
        }
    }
}
