use std::{thread::sleep, time::Duration};

use db_creator::{build, log_db_memory};

fn main() {
    let db = build();
    println!("done");
    log_db_memory(&db);
    sleep(Duration::from_hours(1));
}
