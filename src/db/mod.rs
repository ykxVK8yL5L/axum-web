use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Connection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_db(dbname:String) -> Pool {
    dotenv::dotenv().ok();
    // set up database connection pool
    //let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(dbname);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
