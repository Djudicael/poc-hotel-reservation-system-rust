use payment_service_app::{config::db::migrate_db, *};
use server::starter::start;

#[tokio::main]
async fn main() {
    match migrate_db().await {
        Ok(_) => println!("Migration done"),
        Err(ex) => println!("ERROR - during the database migration {:?}", ex),
    }
    match start().await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("ERROR - web server failed to start. Cause {:?}", ex),
    }
}
