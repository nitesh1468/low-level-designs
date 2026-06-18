#![allow(unused)]
use std::sync::{Mutex,Arc,LazyLock};

struct DatabaseConnection {
}

impl DatabaseConnection {
    fn get_conn() -> &'static Self {
        static CONN:LazyLock<DatabaseConnection> = LazyLock::new(||{
            DatabaseConnection {

            }
        });
        &CONN
    }

    fn query(&self, sql:&str) {
        // Database Operations
    }
}

fn main() {
    let db_conn = DatabaseConnection::get_conn();
    db_conn.query("select * from students;");
}
