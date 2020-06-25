#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use std::env;

use models::*;

fn establish_connection() -> PgConnection {
    let url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost/transaction_dev".to_string());
    PgConnection::establish(&url).expect("Error connecting to database")
}

#[tokio::main]
async fn main() {
    use schema::users::dsl::*;

    let conn = establish_connection();
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let user = users.filter(name.eq("user1")).for_update().first::<User>(&conn).unwrap();

        // compile error
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        println!("{:?}", user);
        Ok(())
    }).unwrap();
}
